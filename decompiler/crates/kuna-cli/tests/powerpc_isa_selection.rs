//! A failed generic PowerPC decode explains the ISA override without guessing it.

use crate::common;

use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::path::Path;
use std::process::Command;

fn image(endian: Endianness, abi: u32, vector_abi: bool) -> Vec<u8> {
    let mut object = Object::new(BinaryFormat::Elf, Architecture::PowerPc64, endian);
    object.flags = FileFlags::Elf {
        os_abi: 0,
        abi_version: 0,
        e_flags: abi,
    };
    let word = |n: u32| match endian {
        Endianness::Big => n.to_be_bytes(),
        Endianness::Little => n.to_le_bytes(),
    };
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    // cmpdi r3,0; iseleq r3,r4,r5; blr; mr r3,r4; blr; invalid word.
    let code: Vec<u8> = [
        0x2c230000, 0x7c64289e, 0x4e800020, 0x7c832378, 0x4e800020, 0,
    ]
    .into_iter()
    .flat_map(word)
    .collect();
    object.append_section_data(text, &code, 4);
    for (name, value, size) in [
        ("select_value", 0, 12),
        ("ordinary", 12, 8),
        ("invalid", 20, 4),
    ] {
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
    if vector_abi {
        let attributes = object.add_section(
            Vec::new(),
            b".gnu.attributes".to_vec(),
            SectionKind::Elf(object::elf::SHT_GNU_ATTRIBUTES),
        );
        // Format A, GNU vendor, Tag_File, Tag_GNU_Power_ABI_Vector=2 (AltiVec).
        let mut data = vec![b'A'];
        data.extend(word(15));
        data.extend(b"gnu\0");
        data.push(1);
        data.extend(word(7));
        data.extend([8, 2]);
        object.append_section_data(attributes, &data, 1);
    }
    object.write().unwrap()
}

fn decompile(path: &Path, function: &str, options: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile",
            path.to_str().unwrap(),
            function,
            "--mode",
            "aggressive",
        ])
        .args(options)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn check_selection_semantics(code: &str) {
    let source = common::scratch_file("powerpc-selection-result", "c");
    let executable = common::scratch_file("powerpc-selection-result", "exe");
    std::fs::write(
        &source,
        format!(
            "#include <stdbool.h>\n{code}\nint main(void) {{\n\
        return select_value(0, 17, 23) != 17 || select_value(1, 17, 23) != 23\n\
            || select_value(-1, 41, 59) != 59 || select_value(0, 41, 59) != 41;\n\
        }}\n"
        ),
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
        "{}",
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
fn generic_powerpc_failure_explains_the_explicit_isa_override() {
    let path = common::scratch_file("powerpc-isa", "o");
    for (endian, name) in [(Endianness::Big, "BE"), (Endianness::Little, "LE")] {
        let generic = format!("PowerPC:{name}:64:default");
        let alternate = format!("PowerPC:{name}:64:A2ALT");
        for (abi, vector) in [(0, false), (1, false), (2, true)] {
            let bytes = image(endian, abi, vector);
            use kuna_sleigh::loadimage::LoadImage;
            let loader = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes_silent(
                "synthetic",
                &bytes,
            )
            .unwrap();
            // A big-endian image that declares ELFv2 takes the ELFv2 compiler spec.
            let compiler = if name == "BE" && abi == 2 { "elfv2" } else { "default" };
            assert_eq!(
                loader.get_arch_type(),
                format!("{generic}:{compiler}").as_bytes()
            );
            std::fs::write(&path, bytes).unwrap();
            for options in [vec![], vec!["--target", generic.as_str()], vec!["--json"]] {
                let output = decompile(&path, "select_value", &options);
                assert!(output.contains("halt_baddata"), "{output}");
                assert!(output.contains("isel encoding"), "{output}");
                assert!(
                    output.contains(&format!("--target {alternate}")),
                    "{output}"
                );
            }
            let output = decompile(&path, "select_value", &["--target", &alternate]);
            check_selection_semantics(&output);
            assert!(
                !output.contains("halt_") && !output.contains("isel encoding"),
                "{output}"
            );
            assert!(
                (output.contains("== 0") || output.contains("if (!a0)"))
                    && output.contains("if (")
                    && output.contains("a1")
                    && output.contains("a2")
                    && output.contains("return"),
                "{output}"
            );
            for function in ["ordinary", "invalid"] {
                let output = decompile(&path, function, &[]);
                assert!(!output.contains("isel encoding"), "{output}");
                assert_eq!(
                    output.contains("halt_baddata"),
                    function == "invalid",
                    "{output}"
                );
                if function == "ordinary" {
                    assert!(output.contains("return a1;"), "{output}");
                }
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn raw_pcode_reports_the_isa_at_the_failed_instruction() {
    let path = common::scratch_file("raw-powerpc-isa", "o");
    for (endian, order) in [(Endianness::Big, "BE"), (Endianness::Little, "LE")] {
        std::fs::write(&path, image(endian, 0, false)).unwrap();
        let alternate = format!("PowerPC:{order}:64:A2ALT");
        let walk = |symbol: &str, options: &[&str], status| {
            let mut args = vec!["raw-pcode", path.to_str().unwrap(), symbol, "--follow", "--json"];
            args.extend_from_slice(options);
            let (out, err, code) = common::run_kuna(&args);
            assert_eq!(code, status, "{err}");
            serde_json::from_str::<serde_json::Value>(&out).unwrap()
        };
        let doc = walk("select_value", &[], 1);
        let issue = &doc["issues"][0];
        assert_eq!(doc["complete"], false);
        assert_eq!(doc["instruction_count"], 1);
        assert_eq!(issue["reason"], "decode_error");
        assert_eq!(issue["address"], doc["start"].as_u64().unwrap() + 4);
        let message = issue["message"].as_str().unwrap();
        for fragment in ["isel", &format!("PowerPC:{order}:64:default"), &format!("--target {alternate}")] {
            assert!(message.contains(fragment), "{message}");
        }
        let (out, _, code) = common::run_kuna(&["raw-pcode", path.to_str().unwrap(), "select_value", "--json"]);
        assert_eq!(code, 1);
        let linear: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(linear["error"]["message"], issue["message"]);
        assert_eq!(linear["error"]["address"], issue["address"]);
        let doc = walk("select_value", &["--target", &alternate], 0);
        assert_eq!(doc["complete"], true);
        assert_eq!(doc["instruction_count"], 3);
        assert!(doc["issues"].as_array().unwrap().is_empty());
        for doc in [walk("invalid", &[], 1), walk("select_value", &["--bytes", "6"], 1)] {
            assert!(!doc["issues"][0]["message"].as_str().unwrap().contains("isel"));
        }
    }
    std::fs::remove_file(path).unwrap();
}
