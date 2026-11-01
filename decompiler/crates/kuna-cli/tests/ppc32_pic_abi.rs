use crate::common;
use object::Object as _;
use std::process::Command;

#[path = "fixtures/ppc32_import_fixture.rs"]
mod fixture;
use fixture::{Fixture, Plt};

#[test]
fn ppc32_pic_import_identity_uses_the_proved_caller_base() {
    for plt in [Plt::Short, Plt::Long] {
        for little in [false, true] {
            for symbolized in [false, true] {
                for got2 in [0, 1, 2] {
                    let mut fixture = Fixture::new(plt);
                    fixture.pic_collision = true;
                    fixture.little = little;
                    fixture.symbolized = symbolized;
                    fixture.missing_got2 = got2 == 0;
                    fixture.got2 = got2 == 2;
                    let bytes = fixture.bytes();
                    let file = object::File::parse(&*bytes).unwrap();
                    let imports =
                        kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file);
                    assert_eq!(
                        imports.len(),
                        1,
                        "{plt:?} little={little} symbolized={symbolized} got2={got2}"
                    );
                    assert_eq!(imports[0].addr, u64::from(fixture.target()));
                    assert_eq!(imports[0].name, b"extra");
                }
            }
        }
    }
}

#[test]
fn ppc32_pic_proof_requires_all_paths_and_callers_to_agree() {
    let call = |pc: u32| 0x48000000 | ((0x1800u32.wrapping_sub(pc)) & 0x03fffffc);
    for little in [false, true] {
        for (middle, proven) in [
            (0x60000000, true),
            (0x3bc02804, true),
            (0x3bc02800, false),
            (0x83c10008, false),
            (0x97de0004, false),
            (0xbb810000, false),
            (0x7fde2214, false),
        ] {
            let mut fixture = Fixture::new(Plt::Short);
            fixture.little = little;
            fixture.pic_collision = true;
            fixture.caller = Some(vec![
                0x3bc02804,
                0x2c030000,
                0x41820008,
                middle,
                call(0x1010),
                0x4e800020,
            ]);
            let bytes = fixture.bytes();
            let file = object::File::parse(&*bytes).unwrap();
            let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file);
            assert_eq!(
                imports.len(),
                usize::from(proven),
                "middle={middle:x} little={little}"
            );
            if proven {
                assert_eq!(imports[0].name, b"extra");
            }
        }
        for second_base in [Some(0x2804), Some(0x2800), None] {
            let mut fixture = Fixture::new(Plt::Short);
            fixture.little = little;
            fixture.pic_collision = true;
            fixture.caller = Some(vec![0x3bc02804, call(0x1004), 0x4e800020]);
            fixture.second_caller = Some(vec![
                second_base.map_or(0x60000000, |base| 0x3bc00000 | base),
                call(0x1010),
                0x4e800020,
            ]);
            let bytes = fixture.bytes();
            let file = object::File::parse(&*bytes).unwrap();
            let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file);
            assert_eq!(
                imports.len(),
                usize::from(second_base == Some(0x2804)),
                "second={second_base:?} little={little}"
            );
        }
    }
}

#[test]
fn ppc32_pic_option_controls_names_contracts_and_xrefs() {
    for noreturn in [false, true] {
        let mut fixture = Fixture::new(Plt::Short);
        fixture.pic_collision = true;
        if noreturn {
            let delta = 0x2800u32 - 0x1014;
            fixture.caller = Some(vec![
                0x7c0802a6,
                0x9421fff0,
                0x90010014,
                0x93c10008,
                0x429f0005,
                0x7fc802a6,
                0x3fde0000,
                0x3bde0000 | delta,
                0x480007e1,
            ]);
        }
        let path = common::scratch_file("ppc32-proved-pic", "so");
        std::fs::write(&path, fixture.bytes()).unwrap();
        let name = if noreturn {
            "__stack_chk_fail"
        } else {
            "extra"
        };
        for option in ["default", "off", "abi"] {
            let enabled = option == "abi";
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command.args([
                "decompile",
                path.to_str().unwrap(),
                "check",
                "--mode",
                "fast",
            ]);
            if option != "default" {
                command.args(["--option", "ppc32picimports", option]);
            }
            let out = command.output().unwrap();
            let text = String::from_utf8_lossy(&out.stdout);
            if noreturn && !enabled {
                assert!(!out.status.success());
                assert!(String::from_utf8_lossy(&out.stderr).contains("Unable to load"));
            } else {
                assert!(
                    out.status.success(),
                    "{text}\n{}",
                    String::from_utf8_lossy(&out.stderr)
                );
            }
            assert_eq!(text.contains(&format!("{name}(")), enabled, "{text}");
            assert_eq!(text.contains("no-return"), enabled && noreturn, "{text}");
            if !noreturn {
                assert!(text.contains("return 7;"), "{text}");
            }
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command.args(["xrefs", path.to_str().unwrap(), "--to", name, "--json"]);
            if option != "default" {
                command.args(["--option", "ppc32picimports", option]);
            }
            let out = command.output().unwrap();
            if enabled {
                assert!(
                    out.status.success(),
                    "{}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let rows: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
                assert!(rows.to_string().contains("0x1020"), "{rows}");
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn ppc32_pic_stage_fixture_matches_authored_bytes() {
    assert_eq!(
        include_bytes!("fixtures/ppc32-pic-indirect.elf").as_slice(),
        fixture::proof_gap("indirect", false)
    );
    let mut fixture = Fixture::new(Plt::Short);
    fixture.pic_collision = true;
    fixture.metadata_padding = 4;
    assert_eq!(
        include_bytes!("fixtures/ppc32-pic-imports.elf").as_slice(),
        fixture.bytes()
    );
}

#[test]
fn ppc32_pic_proof_rejects_missing_bounds_entries_and_relocated_code() {
    use object::ObjectSection;
    for little in [false, true] {
        for mutation in [
            "size",
            "entry",
            "text-relocation",
            "overlap-relocation",
            "address-taken",
            "interior-entry",
        ] {
            let mut fixture = Fixture::new(Plt::Short);
            fixture.pic_collision = true;
            fixture.little = little;
            if mutation == "interior-entry" {
                fixture.second_caller = Some(vec![0x4bffffe0, 0x4e800020]); // b check+0x1c
            }
            let mut bytes = fixture.bytes();
            let file = object::File::parse(&*bytes).unwrap();
            let change = match mutation {
                "size" => Some((
                    file.section_by_name(".dynsym")
                        .unwrap()
                        .file_range()
                        .unwrap()
                        .0 as usize
                        + 2 * 16
                        + 8,
                    0,
                )),
                "entry" => Some((24, 0x101c)),
                "text-relocation" => Some((
                    file.section_by_name(".rela.plt")
                        .unwrap()
                        .file_range()
                        .unwrap()
                        .0 as usize,
                    0x1018,
                )),
                "overlap-relocation" => Some((
                    file.section_by_name(".rela.plt")
                        .unwrap()
                        .file_range()
                        .unwrap()
                        .0 as usize,
                    0xffe,
                )),
                "address-taken" => Some((
                    file.section_by_name(".got")
                        .unwrap()
                        .file_range()
                        .unwrap()
                        .0 as usize,
                    0x1800,
                )),
                _ => None,
            };
            if let Some((offset, value)) = change {
                bytes[offset..offset + 4].copy_from_slice(&fixture.word(value));
            }
            let file = object::File::parse(&*bytes).unwrap();
            let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file);
            // A symbol without a size is still an entry: the proof follows the
            // code, so a missing extent costs nothing.
            if mutation == "size" {
                assert_eq!(imports.len(), 1, "{mutation}, little={little}");
                assert_eq!(imports[0].name, b"extra");
            } else {
                assert!(imports.is_empty(), "{mutation}, little={little}");
            }
        }
    }
}

#[test]
fn ppc32_per_object_bases_resolve_distinct_stubs_independently() {
    for little in [false, true] {
        let mut fixture = Fixture::new(Plt::Short);
        fixture.little = little;
        fixture.pic_collision = true;
        fixture.second_stub = true;
        fixture.caller = Some(vec![0x3bc02804, 0x480007fc, 0x4e800020]);
        fixture.second_caller = Some(vec![0x3bc02808, 0x48000800, 0x4e800020]);
        let bytes = fixture.bytes();
        let file = object::File::parse(&*bytes).unwrap();
        let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file);
        assert_eq!(
            imports
                .iter()
                .map(|s| (s.addr, s.name.as_slice()))
                .collect::<Vec<_>>(),
            vec![(0x1800, b"extra".as_slice()), (0x1810, b"extra".as_slice())]
        );
    }
}

#[test]
fn ppc32_pic_relocation_metadata_is_not_an_address_taken_stub() {
    use object::ObjectSection;
    let mut fixture = Fixture::new(Plt::Short);
    fixture.pic_collision = true;
    let mut bytes = fixture.bytes();
    let file = object::File::parse(&*bytes).unwrap();
    let offset = file
        .section_by_name(".rela.plt")
        .unwrap()
        .file_range()
        .unwrap()
        .0 as usize;
    // An unrelated relocation's packed index/type resembles a stub address.
    // It is not a data pointer and contributes no import slot itself.
    bytes[offset + 4..offset + 8].copy_from_slice(&fixture.word(0x1805));
    let file = object::File::parse(&*bytes).unwrap();
    let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].name, b"extra");
}

#[test]
fn ppc32_pic_dynamic_pointer_relocations_prevent_global_contracts() {
    use object::ObjectSection;
    for relative in [false, true] {
        let mut fixture = Fixture::new(Plt::Short);
        fixture.pic_collision = true;
        let mut bytes = fixture.bytes();
        let file = object::File::parse(&*bytes).unwrap();
        let offset = file
            .section_by_name(".rela.plt")
            .unwrap()
            .file_range()
            .unwrap()
            .0 as usize;
        let relocation = if relative {
            [0x2800, object::elf::R_PPC_RELATIVE, 0x1800]
        } else {
            [0x2800, 0x200 | object::elf::R_PPC_ADDR32, 0x800] // check + 0x800
        };
        bytes[offset..offset + 12].copy_from_slice(&fixture.words(&relocation));
        let file = object::File::parse(&*bytes).unwrap();
        assert!(kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file).is_empty());
    }
}

#[test]
fn ppc32_pic_nested_symbol_overlap_does_not_hide_an_enclosing_function() {
    use object::ObjectSection;
    for little in [false, true] {
        for (outer_size, duplicate_size) in [(4, None), (24, None), (4, Some(24)), (24, Some(4))] {
            let mut fixture = Fixture::new(Plt::Short);
            fixture.little = little;
            fixture.pic_collision = true;
            fixture.caller = Some(vec![
                0x4e800020, 0x4e800020, 0x60000000, 0x3bc02804, 0x480007f0, 0x4e800020,
            ]);
            let mut bytes = fixture.bytes();
            let file = object::File::parse(&*bytes).unwrap();
            let section = file.section_by_name(".dynsym").unwrap();
            let index = section.index().0;
            let mut syms = section.data().unwrap().to_vec();
            syms[40..44].copy_from_slice(&fixture.word(outer_size));
            if let Some(size) = duplicate_size {
                syms.extend(fixture.symbol(0, 0x1000, size, 0x12, 1));
            }
            syms.extend(fixture.symbol(0, 0x1004, 4, 0x12, 1));
            syms.extend(fixture.symbol(0, 0x100c, 12, 0x12, 1));
            let shoff = if little {
                u32::from_le_bytes(bytes[32..36].try_into().unwrap())
            } else {
                u32::from_be_bytes(bytes[32..36].try_into().unwrap())
            } as usize;
            let offset = bytes.len() as u32;
            let size = syms.len() as u32;
            bytes.extend(syms);
            for (field, value) in [(16, offset), (20, size)] {
                let at = shoff + index * 40 + field;
                bytes[at..at + 4].copy_from_slice(&fixture.word(value));
            }
            let file = object::File::parse(&*bytes).unwrap();
            let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file);
            // Every symbol is an entry of its own walk; overlapping extents
            // only widen the scope an unresolved jump would forget.
            assert_eq!(imports.len(), 1, "{outer_size} {duplicate_size:?}");
            assert_eq!(imports[0].name, b"extra");
        }
    }
}

#[test]
fn ppc32_proved_tail_stub_becomes_a_separate_named_function() {
    let mut fixture = Fixture::new(Plt::Short);
    fixture.pic_collision = true;
    fixture.caller = Some(vec![
        0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10008, 0x429f0005, 0x7fc802a6, 0x3fde0000,
        0x3bde17ec, 0x480007e0,
    ]);
    let path = common::scratch_file("ppc32-pic-tail", "so");
    std::fs::write(&path, fixture.bytes()).unwrap();
    for option in ["off", "abi"] {
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "functions",
                path.to_str().unwrap(),
                "--json",
                "--option",
                "ppc32picimports",
                option,
            ])
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let inventory: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        let functions = inventory["functions"].as_array().unwrap();
        assert_eq!(
            functions.len(),
            if option == "abi" { 2 } else { 1 },
            "{inventory}"
        );
        if option == "abi" {
            assert!(functions
                .iter()
                .any(|f| f["address"] == 0x1800 && f["name"] == "__stack_chk_fail"));
        }
    }
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile",
            path.to_str().unwrap(),
            "check",
            "--mode",
            "fast",
            "--option",
            "ppc32picimports",
            "abi",
        ])
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        text.contains("__stack_chk_fail(); // tail-call, no-return"),
        "{text}"
    );
    assert!(
        !text.contains("halt_baddata") && !text.contains("return;"),
        "{text}"
    );
    std::fs::remove_file(path).unwrap();
}

fn assert_proof_gap_refused(name: &str) {
    for little in [false, true] {
        let bytes = fixture::proof_gap(name, little);
        let file = object::File::parse(&*bytes).unwrap();
        assert!(
            kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file).is_empty(),
            "{name} little={little}: ambiguous code must not acquire a global contract"
        );
    }
}

#[test]
fn ppc32_pic_code_derived_indirect_caller_blocks_global_contract() {
    assert_proof_gap_refused("indirect");
    let path = common::scratch_file("ppc32-indirect-contract", "elf");
    std::fs::write(&path, fixture::proof_gap("indirect", false)).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile",
            path.to_str().unwrap(),
            "check2",
            "--mode",
            "fast",
            "--option",
            "ppc32picimports",
            "abi",
        ])
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("return 7;"), "{text}");
    assert!(
        !text.contains("__stack_chk_fail") && !text.contains("no-return"),
        "{text}"
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn ppc32_pic_computed_lr_join_does_not_become_an_abi_return() {
    assert_proof_gap_refused("lr-join");
}

#[test]
fn ppc32_pic_unbounded_lr_writer_cannot_supply_a_return_token() {
    use object::ObjectSection;
    for little in [false, true] {
        let mut f = Fixture::new(Plt::Short);
        f.pic_collision = true;
        f.little = little;
        // check restores this computed LR and r30, then revisits its import call.
        f.second_caller = Some(vec![0x38001020, 0x7c0803a6, 0x3bc02800, 0x4bffffb8]);
        let mut bytes = f.bytes();
        let file = object::File::parse(&*bytes).unwrap();
        let symbols = file.section_by_name(".dynsym").unwrap();
        let end = (symbols.file_range().unwrap().0 + symbols.size()) as usize;
        bytes[end - 8..end - 4].copy_from_slice(&f.word(0));
        let file = object::File::parse(&*bytes).unwrap();
        assert!(kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file).is_empty());
    }
}

#[test]
fn ppc32_pic_padding_entry_contributes_to_the_stub_proof() {
    assert_proof_gap_refused("padding");
}

#[test]
fn ppc32_pic_relocated_stub_does_not_preserve_the_callers_base() {
    assert_proof_gap_refused("relocated");
}

#[test]
fn ppc32_pic_lazy_resolver_never_supplies_or_withholds_a_caller_base() {
    use object::ObjectSection;
    for little in [false, true] {
        for mutation in ["none", "got", "load", "entry"] {
            let mut f = Fixture::new(Plt::Short);
            f.pic_collision = true;
            f.little = little;
            f.lazy_resolver = true;
            f.got_tag = mutation != "got";
            if mutation == "entry" {
                f.second_caller = Some(vec![0x480007fc]); // tail into mtctr, bypassing the GOT load
            }
            let mut bytes = f.bytes();
            if mutation == "load" {
                let file = object::File::parse(&*bytes).unwrap();
                let offset = file
                    .section_by_name(".glink")
                    .unwrap()
                    .file_range()
                    .unwrap()
                    .0 as usize;
                bytes[offset + 48..offset + 52].copy_from_slice(&f.word(0x60000000));
            }
            let file = object::File::parse(&*bytes).unwrap();
            let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file);
            // The resolver's jump leaves for the dynamic linker; it never
            // targets a stub, so a damaged or bypassed resolver changes no
            // caller evidence.
            assert_eq!(imports.len(), 1, "{mutation}, little={little}");
            assert_eq!(imports[0].name, b"extra", "{mutation}, little={little}");
        }
    }
}

fn pic_imports(fixture: &Fixture) -> Vec<(u64, Vec<u8>)> {
    let bytes = fixture.bytes();
    let file = object::File::parse(&*bytes).unwrap();
    kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file)
        .into_iter()
        .map(|s| (s.addr, s.name))
        .collect()
}

fn bl(pc: u32, target: u32) -> u32 {
    0x48000001 | (target.wrapping_sub(pc) & 0x03ff_fffc)
}

#[test]
fn ppc32_pic_unresolved_jump_withdraws_only_the_stubs_its_walk_reaches() {
    // check2: save LR, li r30,0x2808, bl stub2, restore LR, then `ending`.
    let check2 = |ending: &[u32]| {
        let mut words = vec![
            0x7c0802a6,
            0x9421fff0,
            0x90010014,
            0x3bc02808,
            bl(0x1018, 0x1810),
            0x80010014,
            0x38210010,
            0x7c0803a6,
        ];
        words.extend(ending);
        words
    };
    for little in [false, true] {
        let mut fixture = Fixture::new(Plt::Short);
        fixture.little = little;
        fixture.pic_collision = true;
        fixture.second_stub = true;
        // check: li r30,0x2800; b stub1
        fixture.caller = Some(vec![0x3bc02800, 0x480007fc]);
        // An unknown jump (mtctr r3; bctr) withdraws check2's stub only.
        fixture.second_caller = Some(check2(&[0x7c6903a6, 0x4e800420]));
        assert_eq!(
            pic_imports(&fixture),
            vec![(0x1800, b"__stack_chk_fail".to_vec())],
            "little={little}"
        );
        fixture.second_caller = Some(check2(&[0x4e800020]));
        assert_eq!(
            pic_imports(&fixture),
            vec![
                (0x1800, b"__stack_chk_fail".to_vec()),
                (0x1810, b"extra".to_vec())
            ],
            "little={little}"
        );
    }
}

#[test]
fn ppc32_pic_local_calls_keep_r30_only_when_the_callee_restores_it() {
    // check: save LR and r30, set r30, call check2, call the stub, return.
    let caller = vec![
        0x7c0802a6,
        0x9421fff0,
        0x90010014,
        0x93c10008,
        0x3bc02804,
        bl(0x1014, 0x1034),
        bl(0x1018, 0x1800),
        0x80010014,
        0x83c10008,
        0x38210010,
        0x7c0803a6,
        0x4e800020,
        0x60000000,
    ];
    // check2 at 0x1034 sets its own GOT base and branches, so it is
    // summarized rather than run in place; `restores` reloads the saved r30.
    let callee = |restores: bool| {
        vec![
            0x9421fff0,
            0x93c10008,
            0x3bc02800,
            0x2c030000,
            0x41820008,
            0x38600001,
            if restores { 0x83c10008 } else { 0x60000000 },
            0x38210010,
            0x4e800020,
        ]
    };
    for little in [false, true] {
        for restores in [true, false] {
            let mut fixture = Fixture::new(Plt::Short);
            fixture.little = little;
            fixture.pic_collision = true;
            fixture.caller = Some(caller.clone());
            fixture.second_caller = Some(callee(restores));
            let expected = if restores {
                vec![(0x1800, b"extra".to_vec())]
            } else {
                vec![]
            };
            assert_eq!(
                pic_imports(&fixture),
                expected,
                "restores={restores} little={little}"
            );
        }
        // A straight-line helper runs in the caller's state: its r30 reaches
        // the stub, which then reads the other slot.
        let mut fixture = Fixture::new(Plt::Short);
        fixture.little = little;
        fixture.pic_collision = true;
        fixture.caller = Some(caller.clone());
        fixture.second_caller = Some(vec![0x3bc02800, 0x4e800020]);
        assert_eq!(
            pic_imports(&fixture),
            vec![(0x1800, b"__stack_chk_fail".to_vec())],
            "little={little}"
        );
    }
}

#[test]
fn ppc32_pic_jump_table_cases_carry_the_callers_base() {
    // mflr r0; li r30,0x2804; cmplwi r3,1; bgt default; bcl 20,31,.+4;
    // mflr r9; addi r9,r9,0x1c (the table); slwi r3,r3,2; lwzx r10,r9,r3;
    // add r10,r10,r9; mtctr r10; bctr; table: .long 0xc, 0x14 and a word
    // whose target leaves the function; default: mtlr r0; blr;
    // case 1: mtlr r0; b stub.
    let switch = |bounded: bool, dispatch: [u32; 2]| {
        vec![
            0x7c0802a6,
            0x3bc02804,
            if bounded { 0x28030001 } else { 0x60000000 },
            0x41810030,
            0x429f0005,
            0x7d2802a6,
            0x3929001c,
            0x5463103a,
            0x7d49182e,
            0x7d4a4a14,
            dispatch[0],
            dispatch[1],
            0x0000000c,
            0x00000014,
            0x00010000,
            0x7c0803a6,
            0x4e800020,
            0x7c0803a6,
            0x480007b8,
        ]
    };
    for little in [false, true] {
        // A compare bounds the index to its two entries; without it the table
        // is read until the first target outside the function.
        for bounded in [true, false] {
            let mut fixture = Fixture::new(Plt::Short);
            fixture.little = little;
            fixture.pic_collision = true;
            fixture.caller = Some(switch(bounded, [0x7d4903a6, 0x4e800420]));
            assert_eq!(
                pic_imports(&fixture),
                vec![(0x1800, b"extra".to_vec())],
                "bounded={bounded} little={little}"
            );
        }
        // A jump through a register the proof cannot read forgets the base.
        let mut fixture = Fixture::new(Plt::Short);
        fixture.little = little;
        fixture.pic_collision = true;
        fixture.caller = Some(switch(true, [0x7c6903a6, 0x4e800420]));
        assert!(pic_imports(&fixture).is_empty(), "little={little}");
    }
}

#[test]
fn ppc32_pic_driver_linked_library_names_every_stub() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/ppc32-pic-driver.so"
    );
    let bytes = std::fs::read(path).unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let mut imports: Vec<_> =
        kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic_abi(&file)
            .into_iter()
            .map(|s| (s.addr, String::from_utf8(s.name).unwrap()))
            .collect();
    imports.sort();
    assert_eq!(
        imports,
        [
            (0x580, "__stack_chk_fail"),
            (0x590, "other"),
            (0x5a0, "__cxa_finalize"),
            (0x5b0, "another"),
            (0x5c0, "__gmon_start__"),
            (0x5d0, "extra"),
        ]
        .map(|(a, n)| (a, n.to_string()))
    );
}

#[test]
fn abi_mode_is_explicit_and_last_cli_value_wins() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/ppc32-pic-driver.so"
    );
    let run = |options: &[&str]| {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
        cmd.args(["decompile", path, "check", "--mode", "fast"]);
        for option in options {
            cmd.args(["--option", "ppc32picimports", option]);
        }
        let out = cmd.output().unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    };
    let off = run(&["off"]);
    let on = run(&["on"]);
    assert_eq!(off, on);
    assert!(
        on.contains("return 7;") && !on.contains("\n  extra("),
        "{on}"
    );
    let abi = run(&["abi"]);
    assert!(
        abi.contains("\n  extra(") && abi.contains("return 7;"),
        "{abi}"
    );
    assert_eq!(run(&["abi", "on"]), on);
    assert_eq!(run(&["abi", "off"]), off);
    assert_eq!(run(&["off", "abi"]), abi);
}

#[test]
fn counted_branches_select_the_actual_in_function_indirect_target() {
    for little in [false, true] {
        for decrements in 0..=4 {
            let mut fixture = Fixture::new(Plt::Short);
            fixture.little = little;
            fixture.pic_collision = true;
            let mut caller = vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10008, 0x3bc02804, 0x39801040, 0x7d8903a6,
            ];
            caller.extend((0..4).map(|i| {
                if i < decrements {
                    0x42000004
                } else {
                    0x60000000
                }
            }));
            caller.extend([
                0x4e800420,
                0x60000000,
                0x60000000,
                0x60000000,
                0x48000008,
                0x3bc02800,
                bl(0x1044, 0x1800),
                0x83c10008,
                0x80010014,
                0x38210010,
                0x7c0803a6,
                0x38600007,
                0x4e800020,
            ]);
            fixture.caller = Some(caller);
            let expected = if decrements == 0 {
                "__stack_chk_fail"
            } else {
                "extra"
            };
            assert_eq!(
                pic_imports(&fixture),
                vec![(0x1800, expected.as_bytes().to_vec())],
                "decrements={decrements} little={little}"
            );
            if decrements == 4 {
                let path = common::scratch_file("ppc32-counted-target", "so");
                std::fs::write(&path, fixture.bytes()).unwrap();
                let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .args([
                        "decompile",
                        path.to_str().unwrap(),
                        "check",
                        "--mode",
                        "fast",
                        "--option",
                        "ppc32picimports",
                        "abi",
                    ])
                    .output()
                    .unwrap();
                let text = String::from_utf8_lossy(&out.stdout);
                assert!(
                    out.status.success(),
                    "{text}\n{}",
                    String::from_utf8_lossy(&out.stderr)
                );
                assert!(text.contains("extra("), "{text}");
                assert!(text.contains("return 7;"), "{text}");
                assert!(!text.contains("no-return"), "{text}");
                std::fs::remove_file(path).unwrap();
            }
        }
    }
}

/// `check` sets r30 = 0x2804 (stub 0x1800 reads `extra`), `check2` follows it.
/// Each case is code that reaches a stub with a base other than the one the
/// walk would compute; none may name the stub.
fn untracked_paths(caller: Vec<u32>, callee: Vec<u32>, little: bool) -> Vec<(u64, Vec<u8>)> {
    let mut fixture = Fixture::new(Plt::Short);
    fixture.little = little;
    fixture.pic_collision = true;
    fixture.second_stub = true;
    fixture.caller = Some(caller);
    fixture.second_caller = Some(callee);
    pic_imports(&fixture)
}

#[test]
fn ppc32_pic_paths_the_walk_cannot_follow_withhold_the_stub() {
    let prologue = [0x7c0802a6, 0x9421fff0, 0x90010014, 0x3bc02804];
    let with = |body: &[u32]| [&prologue[..], body].concat();
    for little in [false, true] {
        // check2 returns four bytes past its return address, so the walk's
        // `li r30,0x2800` never runs: the stub reads `extra`, not 0x3000.
        let skip = with(&[
            bl(0x1010, 0x1020),
            0x3bc02800,
            bl(0x1018, 0x1800),
            0x48000000,
        ]);
        let skipper = vec![0x7d2802a6, 0x39290004, 0x7d2803a6, 0x4e800020];
        assert!(
            untracked_paths(skip, skipper, little).is_empty(),
            "skip little={little}"
        );

        // `b check2` leaves LR at the return address of `bl check2`, so check2
        // returns into check with r30 = 0x2800 and takes the bne.
        let stale = with(&[
            0x3be00000,
            bl(0x1014, 0x1034),
            0x2c1f0000,
            0x40820010,
            0x3be00001,
            0x3bdefffc,
            0x4800000c,
            bl(0x102c, 0x1800),
            0x48000000,
        ]);
        let leaf = vec![0x2c030000, 0x41820004, 0x4e800020];
        assert!(
            untracked_paths(stale, leaf, little).is_empty(),
            "stale little={little}"
        );

        // check2 saves r30 with stwcx. (no reservation, so no store) or
        // stwbrx (reversed) and reloads the word, which then holds 0x2800 or
        // the swapped base.
        let call = with(&[bl(0x1010, 0x101c), bl(0x1014, 0x1800), 0x48000000]);
        for save in [0x7fc1492d, 0x7fc14d2c] {
            let saver = vec![
                0x9421fff0, 0x39202800, 0x91210008, 0x39200008, save, 0x83c10008, 0x38210010,
                0x2c030000, 0x41820004, 0x4e800020,
            ];
            assert!(
                untracked_paths(call.clone(), saver, little).is_empty(),
                "save={save:#x} little={little}"
            );
        }

        // check2 keeps r30 below its stack pointer across a call; the callee's
        // frame overwrites the word with 0x2800.
        let below = vec![
            0x9421fff0,
            0x7c0802a6,
            0x90010014,
            0x93c1fffc,
            bl(0x102c, 0x1044),
            0x83c1fffc,
            0x80010014,
            0x38210010,
            0x7c0803a6,
            0x4e800020,
            0x9421fff0,
            0x39202800,
            0x9121000c,
            0x38210010,
            0x2c030000,
            0x41820004,
            0x4e800020,
        ];
        assert!(
            untracked_paths(call.clone(), below, little).is_empty(),
            "below little={little}"
        );

        // A bounded table entry of check jumps to stub 0x1810 with r30 =
        // 0x2804 (slot 0x3000), while check2 calls it with r30 = 0x2808.
        let switch = vec![
            0x7c0802a6, 0x3bc02804, 0x28030002, 0x41810030, 0x429f0005, 0x7d2802a6, 0x3929001c,
            0x5463103a, 0x7d49182e, 0x7d4a4a14, 0x7d4903a6, 0x4e800420, 0x0000000c, 0x00000014,
            0x000007e0, 0x7c0803a6, 0x4e800020, 0x7c0803a6, 0x480007b8,
        ];
        assert_eq!(
            untracked_paths(switch, vec![0x3bc02808, 0x480007c0], little),
            vec![(0x1800, b"extra".to_vec())],
            "table little={little}"
        );
    }
}
