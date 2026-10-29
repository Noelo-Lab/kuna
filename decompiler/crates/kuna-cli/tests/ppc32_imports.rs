use crate::common;
use object::Object as _;
use std::process::Command;

#[path = "fixtures/ppc32_import_fixture.rs"]
mod fixture;
use fixture::{Fixture, Plt};

#[test]
fn ppc32_import_names_follow_jump_slots_and_complete_stubs() {
    for plt in [Plt::Bss, Plt::Absolute] {
        for little in [false, true] {
            for symbolized in [false, true] {
                let mut fixture = Fixture::new(plt);
                fixture.little = little;
                fixture.symbolized = symbolized;
                let bytes = fixture.bytes();
                let file = object::File::parse(&*bytes).unwrap();
                assert!(file.dynamic_relocations().unwrap().any(|(slot, _)| slot
                    == if matches!(plt, Plt::Bss) {
                        0x3048
                    } else {
                        0x3000
                    }));
                let imports = kuna_analysis::loader::format::resolve_imports(&file, &bytes);
                assert!(
                    imports
                        .iter()
                        .any(|s| s.addr == u64::from(fixture.target())
                            && s.name == b"__stack_chk_fail"),
                    "{plt:?} little={little} symbolized={symbolized}"
                );
            }
        }
    }
}

#[test]
fn ppc32_no_return_contract_stops_flow_at_the_import() {
    for plt in [Plt::Bss, Plt::Absolute] {
        for symbolized in [false, true] {
            for padded in [false, true] {
                for import in ["__stack_chk_fail", "returning_import"] {
                    let mut fixture = Fixture::new(plt);
                    fixture.symbolized = symbolized;
                    fixture.padded = padded;
                    fixture.import = import;
                    let path = common::scratch_file("ppc32-import", "so");
                    std::fs::write(&path, fixture.bytes()).unwrap();
                    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                        .args([
                            "decompile",
                            path.to_str().unwrap(),
                            "check",
                            "--mode",
                            "fast",
                        ])
                        .output()
                        .unwrap();
                    let text = String::from_utf8_lossy(&out.stdout);
                    assert!(
                        out.status.success(),
                        "{plt:?} {import}: {text}\n{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    assert!(
                        text.contains(&format!("{import}(")) && text.contains("return 7;"),
                        "{plt:?}: {text}"
                    );
                    assert!(
                        !text.contains("halt_baddata") && !text.contains("Bad instruction"),
                        "{text}"
                    );
                    assert_eq!(
                        text.contains("no-return"),
                        import == "__stack_chk_fail",
                        "{text}"
                    );
                    if import == "returning_import" {
                        assert!(text.contains("return 9;"), "{text}");
                    }
                    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                        .args(["xrefs", path.to_str().unwrap(), "--to", import, "--json"])
                        .output()
                        .unwrap();
                    assert!(
                        out.status.success(),
                        "{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let rows: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
                    assert!(rows.to_string().contains("0x1010"), "{rows}");
                    std::fs::remove_file(path).unwrap();
                }
            }
        }
    }
}

#[test]
fn ppc32_declines_unproven_import_contracts() {
    for plt in [Plt::Bss, Plt::Short, Plt::Long, Plt::Absolute] {
        for change in 0..5 {
            if (change >= 2 && matches!(plt, Plt::Bss))
                || (matches!(change, 2 | 3) && matches!(plt, Plt::Absolute))
            {
                continue;
            }
            let mut fixture = Fixture::new(plt);
            match change {
                0 => fixture.relocation = object::elf::R_PPC_ADDR32,
                1 => fixture.addend = 4,
                2 => fixture.got_tag = false,
                3 => fixture.got2 = true,
                _ => fixture.bad_stub = true,
            }
            let bytes = fixture.bytes();
            let file = object::File::parse(&*bytes).unwrap();
            let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file);
            assert!(imports.is_empty(), "{plt:?} change={change}");
        }
    }
}

#[test]
fn ppc32_rejects_malformed_metadata_and_out_of_range_slots() {
    use object::ObjectSection as _;
    for plt in [Plt::Bss, Plt::Short, Plt::Long, Plt::Absolute] {
        for change in 0..10 {
            if (change >= 7 && matches!(plt, Plt::Bss))
                || (matches!(change, 7 | 8) && matches!(plt, Plt::Absolute))
            {
                continue;
            }
            let fixture = Fixture::new(plt);
            let mut bytes = fixture.bytes();
            let file = object::File::parse(&*bytes).unwrap();
            let section = |name| {
                let sec = file.section_by_name(name).unwrap();
                (
                    sec.index().0,
                    sec.file_range().map(|p| p.0 as usize).unwrap_or(0),
                )
            };
            let (plt_index, _) = section(".plt");
            let (_, rela) = section(".rela.plt");
            let (_, sym) = section(".dynsym");
            let (_, got) = section(".got");
            let (_, dynamic) = section(".dynamic");
            let (stub_index, _) = section(".glink");
            let (rela_index, _) = section(".rela.plt");
            let shoff = u32::from_be_bytes(bytes[32..36].try_into().unwrap()) as usize;
            match change {
                0 => bytes[16..18].copy_from_slice(&1u16.to_be_bytes()),
                1 => bytes[shoff + plt_index * 40 + 8..shoff + plt_index * 40 + 12]
                    .copy_from_slice(&0u32.to_be_bytes()),
                2 => bytes[rela..rela + 4].copy_from_slice(&0x3001u32.to_be_bytes()),
                3 => bytes[rela..rela + 4].copy_from_slice(&0x3050u32.to_be_bytes()),
                4 => bytes[sym + 28] = 0x11,
                5 => bytes[shoff + rela_index * 40 + 20..shoff + rela_index * 40 + 24]
                    .copy_from_slice(&0u32.to_be_bytes()),
                6 => {
                    let entry = bytes[rela..rela + 12].to_vec();
                    let off = bytes.len() as u32;
                    bytes.extend(&entry);
                    bytes.extend(&entry);
                    bytes[shoff + rela_index * 40 + 16..shoff + rela_index * 40 + 20]
                        .copy_from_slice(&off.to_be_bytes());
                    bytes[shoff + rela_index * 40 + 20..shoff + rela_index * 40 + 24]
                        .copy_from_slice(&24u32.to_be_bytes());
                }
                7 => bytes[got..got + 4].copy_from_slice(&0xdead0000u32.to_be_bytes()),
                8 => {
                    bytes[dynamic + 16..dynamic + 20].copy_from_slice(&0x70000000u32.to_be_bytes())
                }
                _ => bytes[shoff + stub_index * 40 + 20..shoff + stub_index * 40 + 24]
                    .copy_from_slice(&12u32.to_be_bytes()),
            }
            let file = object::File::parse(&*bytes).unwrap();
            assert!(
                kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file).is_empty(),
                "{plt:?} change={change}"
            );
        }
    }
}

#[test]
fn ppc32_empty_got2_must_not_supply_a_conflicting_slot() {
    use object::ObjectSection as _;
    let fixture = Fixture::new(Plt::Short);
    let mut bytes = fixture.bytes();
    let file = object::File::parse(&*bytes).unwrap();
    let plt = file.section_by_name(".plt").unwrap().index().0;
    let got2 = file.section_by_name(".got2").unwrap().index().0;
    let rela = file.section_by_name(".rela.plt").unwrap();
    let index = rela.index().0;
    let mut entries = rela.data().unwrap().to_vec();
    entries.extend(fixture.words(&[0x3004, 0x115, 0]));
    let shoff = u32::from_be_bytes(bytes[32..36].try_into().unwrap()) as usize;
    let off = bytes.len() as u32;
    bytes.extend(entries);
    for (section, field, value) in [
        (plt, 20, 8),
        (got2, 12, 0xffffa804),
        (index, 16, off),
        (index, 20, 24),
    ] {
        let at = shoff + section * 40 + field;
        bytes[at..at + 4].copy_from_slice(&value.to_be_bytes());
    }
    let file = object::File::parse(&*bytes).unwrap();
    assert!(kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file).is_empty());
}

/// The caller's PC-relative setup establishes r30 = DT_PPC_GOT + 4.
/// Both candidate slot addresses have JMP_SLOTs, but only the wrong one is
/// __stack_chk_fail. Removing .got2 does not change the executed instructions.
#[test]
fn ppc32_caller_r30_cannot_be_replaced_by_dt_ppc_got() {
    for plt in [Plt::Short, Plt::Long] {
        for little in [false, true] {
            for missing_got2 in [false, true] {
                let mut fixture = Fixture::new(plt);
                fixture.pic_collision = true;
                fixture.little = little;
                fixture.missing_got2 = missing_got2;
                let bytes = fixture.bytes();
                let file = object::File::parse(&*bytes).unwrap();
                assert_eq!(file.section_by_name(".got2").is_none(), missing_got2);
                assert_eq!(file.dynamic_relocations().unwrap().count(), 2);
                let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file);
                assert!(
                    !imports.iter().any(|s| s.addr == u64::from(fixture.target())
                        && s.name == b"__stack_chk_fail"),
                    "{plt:?} little={little} missing_got2={missing_got2}: caller uses the other slot"
                );
            }
        }
    }
}

#[test]
fn ppc32_missing_got2_does_not_truncate_returning_calls() {
    for plt in [Plt::Short, Plt::Long] {
        for symbolized in [false, true] {
            let mut fixture = Fixture::new(plt);
            fixture.pic_collision = true;
            fixture.missing_got2 = true;
            fixture.symbolized = symbolized;
            let path = common::scratch_file("ppc32-pic-collision", "so");
            std::fs::write(&path, fixture.bytes()).unwrap();
            let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "decompile",
                    path.to_str().unwrap(),
                    "check",
                    "--mode",
                    "fast",
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
                text.contains("return 7;")
                    && !text.contains("__stack_chk_fail(")
                    && !text.contains("no-return"),
                "{plt:?} symbolized={symbolized}: {text}"
            );
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn ppc32_gnu_discarded_got2_preserves_the_reachable_return() {
    use crate::common::process;
    for tool in ["powerpc-linux-gnu-gcc", "powerpc-linux-gnu-ld"] {
        if process::optional_output(Command::new(tool).arg("--version")).is_none() {
            return;
        }
    }
    let root = common::scratch_file("ppc32-gnu-pic", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("check.c");
    let obj = root.join("check.o");
    std::fs::write(
        &source,
        r#"
extern void __stack_chk_fail(void);
extern void other(void), another(void), extra(void);
void refs(void) { other(); another(); extra(); __stack_chk_fail(); }
int check(void) { extra(); return 7; }
"#,
    )
    .unwrap();
    process::required_output(
        Command::new("powerpc-linux-gnu-gcc")
            .args([
                "-O1",
                "-fPIC",
                "-msecure-plt",
                "-fno-unwind-tables",
                "-fno-asynchronous-unwind-tables",
                "-c",
            ])
            .arg(&source)
            .arg("-o")
            .arg(&obj),
    );
    for linker in ["powerpc-linux-gnu-ld", "ld.lld-21"] {
        if process::optional_output(Command::new(linker).arg("--version")).is_none() {
            continue;
        }
        for stripped in [false, true] {
            let path = root.join(if stripped {
                "stripped.so"
            } else {
                "symbolized.so"
            });
            let mut link = Command::new(linker);
            link.arg("-shared");
            if linker == "powerpc-linux-gnu-ld" {
                link.args(["--secure-plt", "--spare-dynamic-tags=4081"]);
            }
            if stripped {
                link.arg("--strip-all");
            }
            process::required_output(link.arg(&obj).arg("-o").arg(&path));
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            if linker == "powerpc-linux-gnu-ld" {
                assert!(file.section_by_name(".got2").is_none());
            }
            assert_eq!(file.dynamic_relocations().unwrap().count(), 4);
            let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "decompile",
                    path.to_str().unwrap(),
                    "check",
                    "--mode",
                    "fast",
                    "--option",
                    "ppc32picimports",
                    "on",
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
                text.contains("return 7;")
                    && text.contains("extra(")
                    && !text.contains("__stack_chk_fail(")
                    && !text.contains("no-return"),
                "stripped={stripped}: {text}"
            );
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn ppc32_got_metadata_does_not_establish_r30() {
    for plt in [Plt::Short, Plt::Long] {
        for little in [false, true] {
            for symbolized in [false, true] {
                for missing_got2 in [false, true] {
                    let mut fixture = Fixture::new(plt);
                    fixture.little = little;
                    fixture.symbolized = symbolized;
                    fixture.missing_got2 = missing_got2;
                    let bytes = fixture.bytes();
                    let file = object::File::parse(&*bytes).unwrap();
                    assert!(
                        kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file).is_empty(),
                        "{plt:?} little={little} symbolized={symbolized} missing_got2={missing_got2}"
                    );
                }
            }
        }
    }
}

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
                        kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file);
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
    let call = |pc: u32| 0x48000001 | ((0x1800u32.wrapping_sub(pc)) & 0x03fffffc);
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
            let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file);
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
            let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file);
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
        for option in ["default", "off", "on"] {
            let enabled = option == "on";
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
    let mut fixture = Fixture::new(Plt::Short);
    fixture.pic_collision = true;
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
            assert!(
                kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file).is_empty(),
                "{mutation}, little={little}"
            );
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
        fixture.caller = Some(vec![0x3bc02804, 0x480007fd, 0x4e800020]);
        fixture.second_caller = Some(vec![0x3bc02808, 0x48000801, 0x4e800020]);
        let bytes = fixture.bytes();
        let file = object::File::parse(&*bytes).unwrap();
        let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file);
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
    let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file);
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
        assert!(kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file).is_empty());
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
                0x4e800020, 0x4e800020, 0x60000000, 0x3bc02804, 0x480007f1, 0x4e800020,
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
            let imports = kuna_analysis::loader::kuna_ppc32_imports::resolve_with_pic(&file);
            let disjoint = outer_size == 4 && duplicate_size.is_none();
            assert_eq!(imports.len(), usize::from(disjoint));
            if disjoint {
                assert_eq!(imports[0].name, b"extra");
            }
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
    for option in ["off", "on"] {
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
            if option == "on" { 2 } else { 1 },
            "{inventory}"
        );
        if option == "on" {
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
            "on",
        ])
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(text.contains("__stack_chk_fail(); // tail-call, no-return"), "{text}");
    assert!(
        !text.contains("halt_baddata") && !text.contains("return;"),
        "{text}"
    );
    std::fs::remove_file(path).unwrap();
}
