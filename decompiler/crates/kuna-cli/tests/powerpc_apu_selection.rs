use crate::common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

fn powerpc_hard_float_image(little: bool, apu: bool) -> Vec<u8> {
    let endian = if little {
        Endianness::Little
    } else {
        Endianness::Big
    };
    let mut object = Object::new(BinaryFormat::Elf, Architecture::PowerPc, endian);
    let word = |v: u32| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    // fadd f1,f1,f2; blr; cmpwi r3,0; iseleq r3,r4,r5; blr.
    let code: Vec<u8> = [
        0xfc21102a, 0x4e800020, 0x2c030000, 0x7c64289e, 0x4e800020, 0x100004c4, 0x7c0019ce,
        0x4e800020,
    ]
    .into_iter()
    .flat_map(word)
    .collect();
    object.append_section_data(text, &code, 4);
    for (name, value, size) in [("add", 0, 8), ("choose", 8, 12), ("vector", 20, 12)] {
        object.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    if apu {
        let section =
            object.add_section(Vec::new(), b".PPC.EMB.apuinfo".to_vec(), SectionKind::Note);
        let mut note: Vec<u8> = [8, 4, 2].into_iter().flat_map(word).collect();
        note.extend(b"APUinfo\0");
        note.extend(word(0x0040_0001));
        object.append_section_data(section, &note, 4);
    }
    object.write().unwrap()
}

fn check_emitted_c(code: &str, expression: &str) {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("cc is unavailable; skipping only the emitted-C execution check");
        return;
    }
    let source = common::scratch_file("elf-abi-result", "c");
    let executable = common::scratch_file("elf-abi-result", "exe");
    std::fs::write(
        &source,
        format!("{code}\nint main(void) {{ return {expression}; }}\n"),
    )
    .unwrap();
    let output = Command::new("cc")
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{code}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        Command::new(&executable).status().unwrap().success(),
        "{code}"
    );
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(executable).unwrap();
}

#[test]
fn shared_apu_requirements_preserve_altivec_and_hard_float() {
    let path = common::scratch_file("powerpc-apu-hard-float", "o");
    for little in [false, true] {
        for apu in [false, true] {
            std::fs::write(&path, powerpc_hard_float_image(little, apu)).unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .arg("decompile")
                .arg(&path)
                .args(["add", "--mode", "fast"])
                .output()
                .unwrap();
            assert!(output.status.success(), "{output:?}");
            let text = String::from_utf8(output.stdout).unwrap();
            assert!(text.contains("double add(double"), "{text}");
            assert!(text.contains("return a0 + a1;"), "{text}");
            check_emitted_c(&text, "add(1.25, 2.5) != 3.75 || add(-8.5, 0.25) != -8.25");
            let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .arg("decompile")
                .arg(&path)
                .args(["vector", "--mode", "fast"])
                .output()
                .unwrap();
            assert!(output.status.success(), "{output:?}");
            let vector = String::from_utf8(output.stdout).unwrap();
            assert!(!vector.contains("halt_baddata"), "{vector}");
            assert!(vector.contains("= 0;"), "{vector}");
            if apu {
                let endian = if little { "LE" } else { "BE" };
                let target = format!("PowerPC:{endian}:32:e500mc:default");
                let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .arg("decompile")
                    .arg(&path)
                    .args(["choose", "--mode", "fast", "--target", &target])
                    .output()
                    .unwrap();
                assert!(output.status.success(), "{output:?}");
                let text = String::from_utf8(output.stdout).unwrap();
                assert!(!text.contains("halt_baddata"), "{text}");
                let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .arg("decompile")
                    .arg(&path)
                    .args(["add", "--mode", "fast", "--target", &target])
                    .output()
                    .unwrap();
                assert!(output.status.success(), "{output:?}");
                let add = String::from_utf8(output.stdout).unwrap();
                assert!(add.contains("double add(double"), "{add}");
                check_emitted_c(&add, "add(1.25, 2.5) != 3.75");
                check_emitted_c(&text, "choose(0, 17, 23) != 17 || choose(1, 17, 23) != 23 || choose(-1, 41, 59) != 59");
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

fn spe_arguments_image(little: bool, words: &[u32]) -> Vec<u8> {
    let endian = if little {
        Endianness::Little
    } else {
        Endianness::Big
    };
    let mut object = Object::new(BinaryFormat::Elf, Architecture::PowerPc, endian);
    let word = |v: u32| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let code: Vec<u8> = words
        .iter()
        .copied()
        .chain([0x10632200, 0x4e800020])
        .flat_map(word)
        .collect();
    object.append_section_data(text, &code, 4);
    object.add_symbol(Symbol {
        name: b"pick".to_vec(),
        value: 0,
        size: words.len() as u64 * 4,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    let section = object.add_section(Vec::new(), b".PPC.EMB.apuinfo".to_vec(), SectionKind::Note);
    let mut note: Vec<u8> = [8, 4, 2].into_iter().flat_map(word).collect();
    note.extend(b"APUinfo\0");
    note.extend(word(0x0100_0001));
    object.append_section_data(section, &note, 4);
    object.write().unwrap()
}

fn spe_declared(path: &std::path::Path, assertions: &[&str], target: Option<&str>) -> String {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .arg("decompile")
        .arg(path)
        .args(["pick", "--mode", "fast", "--assert-strict"]);
    for assertion in assertions {
        command.args(["--assert", assertion]);
    }
    match target {
        Some(target) => command.args(["--target", target]),
        None => command.args(["--option", "ppcapu", "on"]),
    };
    let output = command.output().unwrap();
    assert!(output.status.success(), "{output:?}");
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn spe_integer_pairs_align_and_spill_without_losing_arguments() {
    let path = common::scratch_file("powerpc-spe-pair", "o");
    for little in [false, true] {
        for padding in [0usize, 1, 2, 5, 6, 7, 8, 9] {
            let mr = |dst: u32, src: u32| 0x7c000378 | (src << 21) | (dst << 16) | (src << 11);
            let first = 3 + ((padding as u32 + 1) & !1);
            let stack = (8 + padding.saturating_sub(8) as u32 * 4 + 7) & !7;
            let words = if first <= 9 {
                vec![mr(3, first), mr(4, first + 1), 0x4e800020]
            } else {
                vec![0x80610000 | stack, 0x80810000 | (stack + 4), 0x4e800020]
            };
            std::fs::write(&path, spe_arguments_image(little, &words)).unwrap();
            let args: String = (0..padding).map(|i| format!("int p{i},")).collect();
            for ty in ["long long", "unsigned long long"] {
                let prototype = format!("prototype pick {ty} pick({args}{ty} x)");
                let text = spe_declared(&path, &[&prototype], None);
                assert!(!text.contains("; //"), "{little} {padding}: {text}");
                let target = format!(
                    "PowerPC:{}:32:e500:default",
                    if little { "LE" } else { "BE" }
                );
                assert_eq!(text, spe_declared(&path, &[&prototype], Some(&target)));
                let values = "11,".repeat(padding);
                check_emitted_c(&text, &format!("pick({values}0x123456789abcdef0LL) != 0x123456789abcdef0LL || pick({values}-1) != ({ty})-1"));
            }
            let words = if first <= 7 {
                vec![mr(3, first + 2), 0x4e800020]
            } else {
                vec![
                    0x80610000 | if first <= 9 { 8 } else { stack + 8 },
                    0x4e800020,
                ]
            };
            std::fs::write(&path, spe_arguments_image(little, &words)).unwrap();
            let prototype = format!("prototype pick int pick({args}long long x,int tail)");
            let text = spe_declared(&path, &[&prototype], None);
            assert!(text.contains("return tail;"), "{little} {padding}: {text}");
            check_emitted_c(
                &text,
                &format!(
                    "pick({}0x123456789abcdef0LL,-7) != -7",
                    "11,".repeat(padding)
                ),
            );
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn spe_aggregate_arguments_remain_indirect() {
    let path = common::scratch_file("powerpc-spe-aggregate", "o");
    for little in [false, true] {
        std::fs::write(
            &path,
            spe_arguments_image(little, &[0x88630000, 0x4e800020]),
        )
        .unwrap();
        for kind in ["struct", "union"] {
            let typedef = format!("typedef {kind} T {{ unsigned char a; }} T;");
            let text = spe_declared(&path, &[&typedef, "prototype pick int pick(T x)"], None);
            assert!(text.contains("x.a"), "{text}");
            check_emitted_c(
                &format!("{typedef}\n{text}\nstatic T x = {{255}};"),
                "pick(x) != 255",
            );
        }
    }
    std::fs::remove_file(path).unwrap();
}

fn spe_caller_image(little: bool, apus: &[u32]) -> Vec<u8> {
    let endian = if little {
        Endianness::Little
    } else {
        Endianness::Big
    };
    let mut object = Object::new(BinaryFormat::Elf, Architecture::PowerPc, endian);
    let word = |v: u32| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    // caller: li r4,2; li r5,3; b add3.  add3: mullw r4,r4,r5; add r3,r3,r4; blr.
    // touch: evaddw r3,r3,r4; blr.
    let code: Vec<u8> = [
        0x38800002, 0x38a00003, 0x48000004, 0x7c8429d6, 0x7c632214, 0x4e800020, 0x10632200,
        0x4e800020,
    ]
    .into_iter()
    .flat_map(word)
    .collect();
    object.append_section_data(text, &code, 4);
    for (name, value, size) in [("caller", 0, 12), ("add3", 12, 12), ("touch", 24, 8)] {
        object.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    let section = object.add_section(Vec::new(), b".PPC.EMB.apuinfo".to_vec(), SectionKind::Note);
    let mut note: Vec<u8> = [8, apus.len() as u32 * 4, 2]
        .into_iter()
        .flat_map(word)
        .collect();
    note.extend(b"APUinfo\0");
    note.extend(apus.iter().copied().flat_map(word));
    object.append_section_data(section, &note, 4);
    object.write().unwrap()
}

fn decompile(path: &std::path::Path, function: &str, args: &[&str]) -> (String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("decompile")
        .arg(path)
        .args([function, "--mode", "fast"])
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    (
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

#[test]
fn spe_requirements_select_e500_only_under_ppcapu() {
    let path = common::scratch_file("powerpc-apu-option", "o");
    for little in [false, true] {
        let endian = if little { "LE" } else { "BE" };
        std::fs::write(&path, spe_caller_image(little, &[0x0100_0001])).unwrap();
        let (text, _) = decompile(&path, "caller", &[]);
        assert!(text.contains("add3(a0,2,3)"), "{text}");
        let generic = format!("PowerPC:{endian}:32:default:default");
        assert_eq!(text, decompile(&path, "caller", &["--target", &generic]).0);
        assert_eq!(text, decompile(&path, "caller", &["--option", "ppcapu", "off"]).0);
        let e500 = format!("PowerPC:{endian}:32:e500:default");
        for function in ["caller", "touch"] {
            assert_eq!(
                decompile(&path, function, &["--option", "ppcapu", "on"]).0,
                decompile(&path, function, &["--target", &e500]).0,
                "{function}"
            );
        }
        let (touch, _) = decompile(&path, "touch", &["--option", "ppcapu", "on"]);
        assert!(!touch.contains("vector"), "{touch}");
        let (touch, _) = decompile(&path, "touch", &[]);
        assert!(touch.contains("vector"), "{touch}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn unclassified_apu_notes_keep_the_generic_decoder_under_ppcapu() {
    let path = common::scratch_file("powerpc-apu-unknown", "o");
    for little in [false, true] {
        let endian = if little { "LE" } else { "BE" };
        for apus in [&[0x0104_0001][..], &[0x0100_0002], &[0x0100_0001, 0xffff_0001]] {
            std::fs::write(&path, spe_caller_image(little, apus)).unwrap();
            let generic = format!("PowerPC:{endian}:32:default:default");
            let (expected, quiet) = decompile(&path, "caller", &["--target", &generic]);
            assert!(!quiet.contains("APU"), "{quiet}");
            let (text, stderr) = decompile(&path, "caller", &["--option", "ppcapu", "on"]);
            assert_eq!(text, expected);
            assert!(stderr.contains("[kuna target]"), "{stderr}");
            assert!(stderr.contains("APU note"), "{stderr}");
            let (_, stderr) = decompile(&path, "caller", &[]);
            assert!(!stderr.contains("APU"), "{stderr}");
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn e500mc_returns_a_64_bit_value_high_word_first() {
    let path = common::scratch_file("powerpc-e500mc-pair", "bin");
    // lis r3,0x1122; ori r3,r3,0x3344; lis r4,0x5566; ori r4,r4,0x7788; blr.
    let code: Vec<u8> = [0x3c601122u32, 0x60633344, 0x3c805566, 0x60847788, 0x4e800020]
        .into_iter()
        .flat_map(u32::to_be_bytes)
        .collect();
    std::fs::write(&path, code).unwrap();
    let (text, _) = decompile(
        &path,
        "0x1000",
        &[
            "--raw-image",
            "--base",
            "0x1000",
            "--target",
            "PowerPC:BE:32:e500mc:default",
        ],
    );
    assert!(text.contains("return 0x1122334455667788;"), "{text}");
    std::fs::remove_file(path).unwrap();
}
