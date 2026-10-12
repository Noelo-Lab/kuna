use crate::common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;
fn image(vfp: Option<u8>) -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let mut words = vec![
        0xeeb77b08u32,
        0xee200b07,
        0xe12fff1e,
        0xeeb70b08,
        0xe12fff1e,
        0xe92d4010,
        0xebfffff8,
        0xeeb67b00,
        0xee300b07,
        0xe8bd8010,
        0xe3a00007,
        0xe12fff1e,
        0xeeb70a08,
        0xe12fff1e,
        0xed900b00,
        0xe12fff1e,
        0xee300b01,
        0xe12fff1e,
        0xe92d4010,
        0xeeb70b08,
        0xe3500000,
        0x08bd8010,
        0xebfffffe,
        0xe7f000f0,
    ];
    for register in 1..=3 {
        words.extend([0xeeb70b08 | (register << 12), 0xe3a00007, 0xe12fff1e]);
    }
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    obj.append_section_data(section, &bytes, 4);
    for (name, value, size) in [
        ("scale", 0, 12),
        ("fixed", 12, 8),
        ("wrapper", 20, 20),
        ("integer_value", 40, 8),
        ("single", 48, 8),
        ("read_value", 56, 8),
        ("add_pair", 64, 8),
        ("error_path", 72, 24),
    ] {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    for register in 1..=3 {
        obj.add_symbol(Symbol {
            name: format!("vfp_scratch_{register}").into_bytes(),
            value: 96 + (register - 1) * 12,
            size: 12,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let failure = obj.add_symbol(Symbol {
        name: b"__stack_chk_fail".to_vec(),
        value: 0,
        size: 0,
        kind: SymbolKind::Unknown,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Undefined,
        flags: SymbolFlags::None,
    });
    obj.add_relocation(
        section,
        object::write::Relocation {
            offset: 88,
            symbol: failure,
            addend: 0,
            flags: object::RelocationFlags::Elf {
                r_type: object::elf::R_ARM_CALL,
            },
        },
    )
    .unwrap();
    if let Some(value) = vfp {
        let section = obj.add_section(Vec::new(), b".ARM.attributes".to_vec(), SectionKind::Other);
        let mut bytes = b"A\x11\0\0\0aeabi\0\x01\x07\0\0\0\x1c".to_vec();
        bytes.push(value);
        obj.append_section_data(section, &bytes, 1);
    }
    obj.write().unwrap()
}
#[test]
fn vfp_results_keep_the_whole_double_and_its_call_contract() {
    let path = common::scratch_file("arm-vfp-returns", "o");
    std::fs::write(&path, image(Some(1))).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args([
        "decompile-all",
        path.to_str().unwrap(),
        "--mode",
        "aggressive",
    ]);
    command.args(["--option", "armfloatreturn", "on"]);
    let result = command.output().unwrap();
    let code = String::from_utf8_lossy(&result.stdout);
    assert!(
        result.status.success(),
        "{code}\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(code.contains("double fixed(void)"), "{code}");
    assert!(code.contains("return 1.5;"), "{code}");
    assert!(code.contains("double scale(double a0)"), "{code}");
    assert!(code.contains("return a0 * 1.5;"), "{code}");
    assert!(code.contains("double wrapper(double a0)"), "{code}");
    assert!(code.contains("scale(a0)"), "{code}");
    assert!(code.contains("unsigned int integer_value(void)"), "{code}");
    assert!(code.contains("return 7;"), "{code}");
    assert!(code.contains("float single(void)"), "{code}");
    assert!(code.contains("double read_value(double *a0)"), "{code}");
    assert!(
        code.contains("double add_pair(double a0,double a1)"),
        "{code}"
    );
    assert!(code.contains("return a0 + a1;"), "{code}");
    assert!(code.contains("__stack_chk_fail();"), "{code}");
    for register in 1..=3 {
        assert!(
            code.contains(&format!(
                "unsigned int vfp_scratch_{register}(void)\n{{\n  return 7;\n}}"
            )),
            "{code}"
        );
    }
    assert!(!code.contains("SUB84"), "{code}");
}
#[test]
fn absent_soft_or_ambiguous_abi_evidence_preserves_option_off_output() {
    for value in [None, Some(0), Some(2), Some(3)] {
        let path = common::scratch_file("arm-non-vfp-returns", "o");
        std::fs::write(&path, image(value)).unwrap();
        let mut results = Vec::new();
        for option in ["off", "on"] {
            let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "decompile-all",
                    path.to_str().unwrap(),
                    "--mode",
                    "aggressive",
                    "--option",
                    "armfloatreturn",
                    option,
                ])
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            results.push(result.stdout);
        }
        assert_eq!(results[0], results[1], "attribute={value:?}");
    }
}
#[test]
fn explicit_integer_storage_and_void_contracts_win_over_inference() {
    let path = common::scratch_file("arm-vfp-explicit", "o");
    std::fs::write(&path, image(Some(1))).unwrap();
    for prototype in [
        "prototype fixed void fixed(void)",
        "prototype fixed unsigned int fixed(void)",
    ] {
        let mut results = Vec::new();
        for option in ["off", "on"] {
            let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "decompile-all",
                    path.to_str().unwrap(),
                    "--mode",
                    "aggressive",
                    "--option",
                    "armfloatreturn",
                    option,
                    "--assert",
                    prototype,
                    "--assert-strict",
                ])
                .output()
                .unwrap();
            let code = String::from_utf8(result.stdout).unwrap();
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            results.push(
                code.split("// Function: fixed @")
                    .nth(1)
                    .unwrap()
                    .split("// Function:")
                    .next()
                    .unwrap()
                    .to_string(),
            );
        }
        assert_eq!(results[0], results[1], "{prototype}");
    }
}

/// Synthetic ARM VFP instructions for narrowing, widening, and full double returns.
fn conversions() -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let words: [u32; 21] = [
        0xee300b00, 0xeeb70bc0, 0xe12fff1e, 0xee300a00, 0xeeb70ac0, 0xe12fff1e, 0xee300b00,
        0xe12fff1e, 0xeeb50bc0, 0xeef1fa10, 0xca000003, 0xeeb70bc0, 0xeef77a00, 0xee300a27,
        0xe12fff1e, 0xee300b00, 0xeeb70bc0, 0xe12fff1e, 0xed900b00, 0xeeb70bc0, 0xe12fff1e,
    ];
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    obj.append_section_data(section, &bytes, 4);
    for (name, value, size) in [
        ("narrow", 0, 12),
        ("widen", 12, 12),
        ("keep_double", 24, 8),
        ("narrow_paths", 32, 40),
        ("narrow_load", 72, 12),
    ] {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let attributes = obj.add_section(Vec::new(), b".ARM.attributes".to_vec(), SectionKind::Other);
    obj.append_section_data(attributes, b"A\x11\0\0\0aeabi\0\x01\x07\0\0\0\x1c\x01", 1);
    obj.write().unwrap()
}

#[test]
fn narrowing_returns_the_final_float_without_stale_double_bytes() {
    let path = common::scratch_file("arm-vfp-conversions", "o");
    std::fs::write(&path, conversions()).unwrap();
    for option in ["off", "on"] {
        let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile-all",
                path.to_str().unwrap(),
                "--mode",
                "aggressive",
                "--option",
                "armfloatreturn",
                option,
            ])
            .output()
            .unwrap();
        let code = String::from_utf8_lossy(&result.stdout);
        assert!(
            result.status.success(),
            "{code}\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(code.contains("float narrow("), "{option}: {code}");
        assert!(code.contains("float narrow_paths("), "{option}: {code}");
        assert!(
            code.contains("float narrow_load(double *a0)"),
            "{option}: {code}"
        );
        assert!(code.contains("return (float)*a0;"), "{option}: {code}");
        assert!(!code.contains("CONCAT44"), "{option}: {code}");
        if option == "on" {
            assert!(code.contains("float narrow(double a0)"), "{code}");
            assert!(code.contains("return (float)(a0 + a0);"), "{code}");
            assert!(code.contains("float narrow_paths(double a0)"), "{code}");
            assert!(code.contains("return (float)a0 + 1.0;"), "{code}");
            assert!(code.contains("double widen(float a0)"), "{code}");
            assert!(code.contains("return (double)(a0 + a0);"), "{code}");
            assert!(code.contains("double keep_double(double a0)"), "{code}");
            assert!(code.contains("return a0 + a0;"), "{code}");
        }
    }
}

#[test]
fn explicit_narrowing_output_contracts_remain_authoritative() {
    let path = common::scratch_file("arm-vfp-narrow-explicit", "o");
    std::fs::write(&path, conversions()).unwrap();
    for result_type in ["float", "double", "void"] {
        let prototype = format!("prototype narrow {result_type} narrow(double)");
        let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile",
                path.to_str().unwrap(),
                "narrow",
                "--mode",
                "aggressive",
                "--option",
                "armfloatreturn",
                "on",
                "--assert",
                &prototype,
                "--assert-strict",
            ])
            .output()
            .unwrap();
        let code = String::from_utf8_lossy(&result.stdout);
        assert!(
            result.status.success(),
            "{code}\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(
            code.contains(&format!("{result_type} narrow(")),
            "{prototype}: {code}"
        );
    }
}

/// Instructions compiled from fixtures/mixed.c with GCC -O2 -marm -mfpu=vfpv3-d16.
fn mixed_returns() -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let words: [u32; 41] = [
        0xeeb50bc0, 0xeef1fa10, 0xda000001, 0xeeb70a00, 0xe12fff1e, 0xee200b00, 0xeeb70bc0,
        0xe12fff1e, 0xeeb50bc0, 0xeef1fa10, 0xda000001, 0xed900a00, 0xe12fff1e, 0xee200b00,
        0xeeb70bc0, 0xe12fff1e, 0xeeb50bc0, 0xeef1fa10, 0xde200b00, 0xdeb71bc0, 0xeeb00a41,
        0xe12fff1e, 0xeeb50bc0, 0xeef1fa10, 0xda000001, 0xeeb70b00, 0xe12fff1e, 0xee200b00,
        0xe12fff1e, 0xeeb50bc0, 0xeef1fa10, 0xda000001, 0xed900b00, 0xe12fff1e, 0xee200b00,
        0xe12fff1e, 0xeeb50bc0, 0xeef1fa10, 0xde201b00, 0xeeb00b41, 0xe12fff1e,
    ];
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    obj.append_section_data(section, &bytes, 4);
    for (name, value, size) in [
        ("mixed_constant", 0, 32),
        ("mixed_load", 32, 32),
        ("mixed_input", 64, 24),
        ("double_constant", 88, 28),
        ("double_load", 116, 28),
        ("double_input", 144, 20),
    ] {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let attributes = obj.add_section(Vec::new(), b".ARM.attributes".to_vec(), SectionKind::Other);
    obj.append_section_data(attributes, b"A\x11\0\0\0aeabi\0\x01\x07\0\0\0\x1c\x01", 1);
    obj.write().unwrap()
}

#[test]
fn mixed_paths_preserve_float_constants_loads_and_copied_inputs() {
    let path = common::scratch_file("arm-vfp-mixed-returns", "o");
    std::fs::write(&path, mixed_returns()).unwrap();
    for option in ["off", "on"] {
        let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile-all",
                path.to_str().unwrap(),
                "--mode",
                "aggressive",
                "--option",
                "armfloatreturn",
                option,
            ])
            .output()
            .unwrap();
        let code = String::from_utf8_lossy(&result.stdout);
        assert!(
            result.status.success(),
            "{code}\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        for name in ["mixed_constant", "mixed_load", "mixed_input"] {
            assert!(code.contains(&format!("float {name}(")), "{option}: {code}");
        }
        assert!(!code.contains("CONCAT44"), "{option}: {code}");
        assert!(code.contains("return 1.0;"), "{option}: {code}");
        if option == "on" {
            for signature in [
                "float mixed_constant(double a0)",
                "float mixed_load(double a0,float *a1)",
                "float mixed_input(double a0,float a1)",
                "double double_constant(double a0)",
                "double double_load(double a0,double *a1)",
                "double double_input(double a0,double a1)",
            ] {
                assert!(code.contains(signature), "{code}");
            }
            assert!(code.contains("return (float)(a0 * a0);"), "{code}");
            assert!(code.contains("return *a1;"), "{code}");
            assert!(code.contains("return a1;"), "{code}");
            assert!(code.contains("return a0 * a0;"), "{code}");
        }
    }
}

#[test]
fn explicit_mixed_path_output_contracts_remain_authoritative() {
    let path = common::scratch_file("arm-vfp-mixed-explicit", "o");
    std::fs::write(&path, mixed_returns()).unwrap();
    for name in ["mixed_constant", "mixed_load", "mixed_input"] {
        for result_type in ["float", "double", "void"] {
            let args = match name {
                "mixed_load" => "double, float *",
                "mixed_input" => "double, float",
                _ => "double",
            };
            let prototype = format!("prototype {name} {result_type} {name}({args})");
            let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "decompile",
                    path.to_str().unwrap(),
                    name,
                    "--mode",
                    "aggressive",
                    "--option",
                    "armfloatreturn",
                    "on",
                    "--assert",
                    &prototype,
                    "--assert-strict",
                ])
                .output()
                .unwrap();
            let code = String::from_utf8_lossy(&result.stdout);
            assert!(
                result.status.success(),
                "{code}\n{}",
                String::from_utf8_lossy(&result.stderr)
            );
            assert!(
                code.contains(&format!("{result_type} {name}(")),
                "{prototype}: {code}"
            );
        }
    }
}

/// Synthetic ARM VFP returns from docs/features/armfloatreturn/fixtures/constants.c.
fn constant_returns() -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let words: [u32; 64] = [
        0xee200b00, 0xeeb76b08, 0xeeb77a00, 0xeef07a00, 0xeeb40bc6, 0xeef1fa10, 0xdeb00a67,
        0xceb00a47, 0xe12fff1e, 0xee200b00, 0xeeb77b08, 0xeeb40bc7, 0xeef1fa10, 0xda000001,
        0xed900a00, 0xe12fff1e, 0xed900a01, 0xe12fff1e, 0xee200b00, 0xeeb77b08, 0xeeb40bc7,
        0xeef1fa10, 0xdeb00a61, 0xceb00a41, 0xe12fff1e, 0xee200b00, 0xeeb75b08, 0xeeb76b00,
        0xeeb07b00, 0xeeb40bc5, 0xeef1fa10, 0xdeb00b47, 0xceb00b46, 0xe12fff1e, 0xee200b00,
        0xeeb77b08, 0xeeb40bc7, 0xeef1fa10, 0xda000001, 0xed900b00, 0xe12fff1e, 0xed900b02,
        0xe12fff1e, 0xee200b00, 0xeeb77b08, 0xeeb40bc7, 0xeef1fa10, 0xdeb00b42, 0xceb00b41,
        0xe12fff1e, 0xee200b00, 0xeeb77b08, 0xeeb40bc7, 0xeef1fa10, 0xda000002, 0xe2803102,
        0xee003a10, 0xe12fff1e, 0xeeb00a00, 0xe12fff1e, 0xee200b00, 0xed9f0a00, 0xe12fff1e,
        0x00000000,
    ];
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    obj.append_section_data(section, &bytes, 4);
    for (name, value, size) in [
        ("constant_paths", 0, 36),
        ("loaded_paths", 36, 36),
        ("copied_paths", 72, 28),
        ("double_paths", 100, 36),
        ("double_loaded_paths", 136, 36),
        ("double_copied_paths", 172, 28),
        ("integer_bits", 200, 40),
        ("double_low_bits", 240, 16),
    ] {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let attributes = obj.add_section(Vec::new(), b".ARM.attributes".to_vec(), SectionKind::Other);
    obj.append_section_data(attributes, b"A\x11\0\0\0aeabi\0\x01\x07\0\0\0\x1c\x01", 1);
    obj.write().unwrap()
}

#[test]
fn partial_returns_keep_four_bytes_without_float_arithmetic() {
    let path = common::scratch_file("arm-vfp-partial-returns", "o");
    std::fs::write(&path, constant_returns()).unwrap();
    for option in ["off", "on"] {
        let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile-all",
                path.to_str().unwrap(),
                "--mode",
                "aggressive",
                "--option",
                "armfloatreturn",
                option,
            ])
            .output()
            .unwrap();
        let code = String::from_utf8_lossy(&result.stdout);
        assert!(
            result.status.success(),
            "{code}\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        for name in [
            "constant_paths",
            "loaded_paths",
            "copied_paths",
            "integer_bits",
        ] {
            let body = code
                .split(&format!("// Function: {name} @"))
                .nth(1)
                .unwrap()
                .split("// Function:")
                .next()
                .unwrap();
            if option == "on" {
                assert!(body.contains(&format!("float {name}(")), "{option}: {body}");
            } else {
                assert!(
                    !body.contains(&format!("double {name}(")),
                    "{option}: {body}"
                );
            }
            assert!(!body.contains("CONCAT44"), "{option}: {body}");
        }
        if option == "on" {
            assert!(code.contains("1.0"), "{code}");
            assert!(code.contains("2.0"), "{code}");
            for name in ["double_paths", "double_loaded_paths", "double_copied_paths"] {
                assert!(code.contains(&format!("double {name}(")), "{code}");
            }
        }
    }
}

#[test]
fn declared_double_with_a_partial_word_edit_keeps_all_eight_bytes() {
    let path = common::scratch_file("arm-vfp-double-word-edit", "o");
    std::fs::write(&path, constant_returns()).unwrap();
    for option in ["off", "on"] {
        let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile",
                path.to_str().unwrap(),
                "double_low_bits",
                "--mode",
                "aggressive",
                "--option",
                "armfloatreturn",
                option,
                "--assert",
                "prototype double_low_bits double double_low_bits(double)",
                "--assert-strict",
            ])
            .output()
            .unwrap();
        let code = String::from_utf8_lossy(&result.stdout);
        assert!(
            result.status.success(),
            "{code}\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(code.contains("double double_low_bits(double a0)"), "{code}");
        assert!(code.contains("0xffffffff00000000"), "{code}");
        assert!(code.contains("a0 * a0"), "{code}");
    }
}

/// A VFP parameter passed to a first call leaves a second call its own argument.
#[test]
fn a_later_call_keeps_the_argument_computed_in_the_same_register() {
    let fixture = common::fixture("armfloatreturn_armhf.o");
    for option in ["off", "on"] {
        let (code, err, rc) = common::run_kuna(&[
            "decompile-all",
            &fixture,
            "--mode",
            "aggressive",
            "--option",
            "armfloatreturn",
            option,
        ]);
        assert_eq!(rc, 0, "{err}");
        assert!(code.contains("float twocallsf(float a0)"), "{option}: {code}");
        assert!(code.contains("return scalef(a0 * 3.0) + 1.0;"), "{option}: {code}");
        if option == "on" {
            assert!(code.contains("double twocalls(double a0)"), "{code}");
            assert!(code.contains("  scale(a0);\n"), "{code}");
            assert!(code.contains("return scale(a0 * 3.0) + 1.0;"), "{code}");
        }
    }
}

/// A double a call left in d0 does not displace the int computed in r0, an
/// unused d0 below a used d1 is one double, and two floats read as the halves
/// of d0 are not one 8-byte parameter.
#[test]
fn an_int_return_beats_a_double_left_in_d0_by_a_call() {
    let fixture = common::fixture("armfloatreturn_armhf.o");
    for option in ["off", "on"] {
        let (code, err, rc) = common::run_kuna(&[
            "decompile-all",
            &fixture,
            "--mode",
            "aggressive",
            "--option",
            "armfloatreturn",
            option,
        ]);
        assert_eq!(rc, 0, "{err}");
        assert!(!code.contains("w2(unsigned long long"), "{option}: {code}");
        if option == "on" {
            assert!(code.contains("double w2(void)"), "{code}");
            assert!(code.contains("int bump(double a0,int a1)"), "{code}");
            assert!(code.contains("  half(a0);\n  return a1 + 1;"), "{code}");
            assert!(code.contains("double second(double a0,double a1)"), "{code}");
        } else {
            assert!(code.contains("int bump(int a0)"), "{code}");
            assert!(code.contains("return a0 + 1;"), "{code}");
        }
    }
}

/// A double one path takes from a call and another computes is returned on both.
#[test]
fn a_double_returned_from_a_call_on_one_path_keeps_the_other_path() {
    let fixture = common::fixture("armfloatreturn_armhf.o");
    let (code, err, rc) = common::run_kuna(&[
        "decompile-all",
        &fixture,
        "--mode",
        "aggressive",
        "--option",
        "armfloatreturn",
        "on",
    ]);
    assert_eq!(rc, 0, "{err}");
    let body = |name: &str| {
        code.split(&format!("// Function: {name} @"))
            .nth(1)
            .and_then(|b| b.split("// Function:").next())
            .unwrap_or_default()
            .to_string()
    };
    let a3 = body("a3");
    assert!(a3.contains("double a3(double a0)"), "{a3}");
    assert!(a3.contains("half(a0)"), "{a3}");
    assert!(a3.contains("return a0 * 3.0;"), "{a3}");
    let a7 = body("a7");
    assert!(a7.contains("double a7(double a0)"), "{a7}");
    assert!(a7.contains("return 2.5;"), "{a7}");
    let a6 = body("a6");
    assert!(a6.contains("double a6(double a0,int a1)"), "{a6}");
    assert!(a6.contains("a0 * 3.0"), "{a6}");
}
