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
            let imports = kuna_analysis::loader::format::resolve_imports(&file, &bytes);
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
                kuna_analysis::loader::format::resolve_imports(&file, &bytes).is_empty(),
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
    assert!(kuna_analysis::loader::format::resolve_imports(&file, &bytes).is_empty());
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
                let imports = kuna_analysis::loader::format::resolve_imports(&file, &bytes);
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
    for stripped in [false, true] {
        let path = root.join(if stripped {
            "stripped.so"
        } else {
            "symbolized.so"
        });
        let mut link = Command::new("powerpc-linux-gnu-ld");
        link.args(["-shared", "--secure-plt", "--spare-dynamic-tags=4081"]);
        if stripped {
            link.arg("--strip-all");
        }
        process::required_output(link.arg(&obj).arg("-o").arg(&path));
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        assert!(file.section_by_name(".got2").is_none());
        assert_eq!(file.dynamic_relocations().unwrap().count(), 4);
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
            "stripped={stripped}: {text}"
        );
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
                        kuna_analysis::loader::format::resolve_imports(&file, &bytes).is_empty(),
                        "{plt:?} little={little} symbolized={symbolized} missing_got2={missing_got2}"
                    );
                }
            }
        }
    }
}

#[test]
fn ppc32_shared_unwind_keeps_unnamed_stubs_in_text() {
    use object::ObjectSection;
    use kuna_analysis::analyzers::entry::kuna_fdeinterior::eligible_fde_bodies;
    for (plt, little) in [Plt::Short, Plt::Long].into_iter().flat_map(|p| [false, true].map(|le| (p, le))) {
        let mut f = Fixture::new(plt);
        f.unwind_only = true;
        f.second_stub = true;
        f.little = little;
        let mut bytes = f.bytes();
        let file = object::File::parse(bytes.as_slice()).unwrap();
        let stub = file.section_by_name(".glink").unwrap().file_range().unwrap().0 as usize;
        let frame = file.section_by_name(".eh_frame").unwrap().file_range().unwrap().0 as usize;
        assert_eq!(eligible_fde_bodies(&file, &bytes), vec![(0x1000, 0x1014)]);
        // The linker may merge .glink into .text, with one FDE for all stubs.
        let name = bytes.windows(7).position(|b| b == b".glink\0").unwrap();
        bytes[name..name + 7].copy_from_slice(b".text\0\0");
        bytes[frame + 28..frame + 32].copy_from_slice(&f.word(0x1800u32.wrapping_sub(0x601c)));
        bytes[frame + 32..frame + 36].copy_from_slice(&f.word(32));
        let file = object::File::parse(bytes.as_slice()).unwrap();
        assert!(eligible_fde_bodies(&file, &bytes).is_empty(), "{plt:?} little={little}");
        let ctr = if matches!(plt, Plt::Short) { 4 } else { 8 };
        for off in [stub + ctr, stub + 16 + ctr] {
            bytes[off..off + 4].copy_from_slice(&f.word(0x60000000));
        }
        let file = object::File::parse(bytes.as_slice()).unwrap();
        assert_eq!(eligible_fde_bodies(&file, &bytes), vec![(0x1800, 0x1820)]);
    }
}


#[test]
fn ppc32_pic_stage_fixture_matches_authored_bytes() {
    let (original, swapped) = fixture::cache_pair();
    assert_eq!(
        include_bytes!("fixtures/ppc32-pic-cache-original.elf").as_slice(),
        original
    );
    assert_eq!(
        include_bytes!("fixtures/ppc32-pic-cache-swapped.elf").as_slice(),
        swapped
    );
    assert_eq!(
        include_bytes!("fixtures/ppc32-pic-abi.elf").as_slice(),
        fixture::abi_caller()
    );
    assert_eq!(
        include_bytes!("fixtures/ppc32-pic-indirect.elf").as_slice(),
        fixture::proof_gap("indirect", false)
    );
    for (name, bytes) in [
        (
            "composed-words",
            include_bytes!("fixtures/ppc32-pic-composed-words.elf").as_slice(),
        ),
        (
            "vector-copy",
            include_bytes!("fixtures/ppc32-pic-vector-copy.elf").as_slice(),
        ),
        (
            "mapped-data",
            include_bytes!("fixtures/ppc32-pic-mapped-data.elf").as_slice(),
        ),
        (
            "note-valid",
            include_bytes!("fixtures/ppc32-pic-note-valid.elf").as_slice(),
        ),
        (
            "string-store",
            include_bytes!("fixtures/ppc32-pic-string-store.elf").as_slice(),
        ),
        (
            "split-store",
            include_bytes!("fixtures/ppc32-pic-split-store.elf").as_slice(),
        ),
        (
            "unaligned-data",
            include_bytes!("fixtures/ppc32-pic-unaligned-data.elf").as_slice(),
        ),
        (
            "callee-output",
            include_bytes!("fixtures/ppc32-pic-callee-output.elf").as_slice(),
        ),
        (
            "float-copy",
            include_bytes!("fixtures/ppc32-pic-float-copy.elf").as_slice(),
        ),
        (
            "unsupported-rfi",
            include_bytes!("fixtures/ppc32-pic-unsupported-rfi.elf").as_slice(),
        ),
        (
            "unsupported-tar",
            include_bytes!("fixtures/ppc32-pic-unsupported-tar.elf").as_slice(),
        ),
        (
            "callee-callback",
            include_bytes!("fixtures/ppc32-pic-callee-callback.elf").as_slice(),
        ),
        (
            "return-export",
            include_bytes!("fixtures/ppc32-pic-return-export.elf").as_slice(),
        ),
        (
            "callback-shift",
            include_bytes!("fixtures/ppc32-pic-callback-shift.elf").as_slice(),
        ),
        (
            "callback-call-clobber",
            include_bytes!("fixtures/ppc32-pic-callback-call-clobber.elf").as_slice(),
        ),
        (
            "tail-callback",
            include_bytes!("fixtures/ppc32-pic-tail-callback.elf").as_slice(),
        ),
        (
            "local-alias",
            include_bytes!("fixtures/ppc32-pic-local-alias.elf").as_slice(),
        ),
        (
            "unproved-stub",
            include_bytes!("fixtures/ppc32-pic-unproved-stub.elf").as_slice(),
        ),
        (
            "dynamic-init",
            include_bytes!("fixtures/ppc32-pic-dynamic-init.elf").as_slice(),
        ),
        (
            "lr-alias-returning",
            include_bytes!("fixtures/ppc32-pic-lr-alias-returning.elf").as_slice(),
        ),
        (
            "shift-spill",
            include_bytes!("fixtures/ppc32-pic-shift-spill.elf").as_slice(),
        ),
        (
            "stack-bytes",
            include_bytes!("fixtures/ppc32-pic-stack-bytes.elf").as_slice(),
        ),
        (
            "nonexec-section",
            include_bytes!("fixtures/ppc32-pic-nonexec-section.elf").as_slice(),
        ),
        (
            "init-array",
            include_bytes!("fixtures/ppc32-pic-init-array.elf").as_slice(),
        ),
        (
            "literal-halfword",
            include_bytes!("fixtures/ppc32-pic-literal-halfword.elf").as_slice(),
        ),
        (
            "store-word-halfword",
            include_bytes!("fixtures/ppc32-pic-store-word-halfword.elf").as_slice(),
        ),
        (
            "stub-interior",
            include_bytes!("fixtures/ppc32-pic-stub-interior.elf").as_slice(),
        ),
        (
            "unaligned-exec",
            include_bytes!("fixtures/ppc32-pic-unaligned-exec.elf").as_slice(),
        ),
        (
            "literal",
            include_bytes!("fixtures/ppc32-pic-literal.elf").as_slice(),
        ),
        (
            "resolver",
            include_bytes!("fixtures/ppc32-pic-resolver.elf").as_slice(),
        ),
        (
            "store-halfword",
            include_bytes!("fixtures/ppc32-pic-store-halfword.elf").as_slice(),
        ),
        (
            "internal-link",
            include_bytes!("fixtures/ppc32-pic-internal-link.elf").as_slice(),
        ),
        (
            "indirect-chain",
            include_bytes!("fixtures/ppc32-pic-indirect-chain.elf").as_slice(),
        ),
        (
            "base-spill",
            include_bytes!("fixtures/ppc32-pic-base-spill.elf").as_slice(),
        ),
        (
            "ctr-decrement",
            include_bytes!("fixtures/ppc32-pic-ctr-decrement.elf").as_slice(),
        ),
        (
            "indirect-branch",
            include_bytes!("fixtures/ppc32-pic-indirect-branch.elf").as_slice(),
        ),
        (
            "local-fallthrough",
            include_bytes!("fixtures/ppc32-pic-local-fallthrough.elf").as_slice(),
        ),
    ] {
        assert_eq!(bytes, fixture::proof_gap(name, false));
    }
    let mut unwind = Fixture::new(Plt::Short);
    unwind.metadata_padding = 4;
    unwind.pic_collision = true;
    unwind.unwind_only = true;
    assert_eq!(
        include_bytes!("fixtures/ppc32-pic-unwind.elf").as_slice(),
        unwind.bytes()
    );
    let mut fixture = Fixture::new(Plt::Short);
    fixture.metadata_padding = 4;
    fixture.pic_collision = true;
    assert_eq!(
        include_bytes!("fixtures/ppc32-pic-imports.elf").as_slice(),
        fixture.bytes()
    );
}
