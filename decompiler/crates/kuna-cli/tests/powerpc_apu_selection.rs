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
