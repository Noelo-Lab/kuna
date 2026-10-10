//! Parameter assertions must preserve the value read at function entry.
use crate::common;

use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::path::PathBuf;
use std::process::Command;

const ADD: &[u8] = &[0x8d, 0x46, 0x01, 0xc3];
const SYSV_CLEAR: &[u8] = &[0x83, 0xfe, 0x07, 0x75, 0x02, 0x31, 0xf6, 0x89, 0xf0, 0xc3];
const MSABI_CLEAR: &[u8] = &[0x83, 0xfa, 0x07, 0x75, 0x02, 0x31, 0xd2, 0x89, 0xd0, 0xc3];
const PROTO: &str = "prototype probe int probe(void *unused,int value)";
const MS_PROTO: &str = "prototype probe int MSABI probe(void *unused,int value)";

fn image(bytes: &[u8]) -> PathBuf {
    image_with_callee(bytes, None)
}

fn image_with_callee(bytes: &[u8], callee: Option<usize>) -> PathBuf {
    let mut object = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    object.append_section_data(text, bytes, 1);
    object.add_symbol(Symbol {
        name: b"probe".to_vec(),
        value: 0,
        size: callee.unwrap_or(bytes.len()) as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    if let Some(offset) = callee {
        object.add_symbol(Symbol {
            name: b"sink".to_vec(),
            value: offset as u64,
            size: (bytes.len() - offset) as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    let path = common::scratch_file("parameter-name", "o");
    std::fs::write(&path, object.write().unwrap()).unwrap();
    path
}

fn run(
    image: &PathBuf,
    prototype: &str,
    assertions: &[&str],
    json: bool,
    batch: bool,
) -> (String, String, i32) {
    let specs = common::repo_root().join("specs");
    let mut args = if batch {
        vec![
            "decompile-all",
            image.to_str().unwrap(),
            "--functions",
            "probe",
        ]
    } else {
        vec!["decompile", image.to_str().unwrap(), "probe"]
    };
    args.extend([
        "--assert-strict",
        "--mode",
        "reliable",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--assert",
        prototype,
    ]);
    for assertion in assertions {
        args.extend(["--assert", assertion]);
    }
    if json {
        args.push("--json");
    }
    common::run_kuna(&args)
}

fn decompile(image: &PathBuf, prototype: &str, assertions: &[&str]) -> String {
    let mut code = None;
    for (json, batch) in [(false, false), (true, false), (true, true)] {
        let (stdout, stderr, status) = run(image, prototype, assertions, json, batch);
        assert_eq!(status, 0, "{stderr}\n{stdout}");
        let actual = if json {
            let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
            for outcome in doc["assertions"].as_array().unwrap() {
                assert_eq!(outcome["status"], "applied", "{outcome}");
            }
            doc["functions"][0]["code"].as_str().unwrap().to_string()
        } else {
            stdout
        };
        if let Some(expected) = code.as_ref() {
            assert_eq!(actual.trim(), expected, "surfaces disagree");
        } else {
            code = Some(actual.trim().to_string());
        }
    }
    code.unwrap()
}

fn semantic_oracle(image: &PathBuf, emitted: &[String], msabi: bool) {
    semantic_oracle_with_callee(image, emitted, msabi, "");
}

fn semantic_oracle_with_callee(image: &PathBuf, emitted: &[String], msabi: bool, support: &str) {
    let driver = common::scratch_file("parameter-driver", "c");
    let source = common::scratch_file("parameter-emitted", "c");
    let native = common::scratch_file("parameter-native", "exe");
    let rebuilt = common::scratch_file("parameter-rebuilt", "exe");
    let harness = "#include <stdio.h>\nint main(void) { int values[] = {-10,-1,0,1,7,11,42,2147483646}; for(unsigned i=0;i<sizeof(values)/sizeof(values[0]);i++) printf(\"%d\\n\",probe(0,values[i])); return 0; }\n";
    let abi = if msabi { "__attribute__((ms_abi))" } else { "" };
    std::fs::write(
        &driver,
        format!("extern int {abi} probe(void *,int);\n{harness}"),
    )
    .unwrap();
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        process::required_output(
            Command::new(cc)
                .arg(&driver)
                .arg(image)
                .arg("-o")
                .arg(&native),
        );
        let expected = process::required_output(&mut Command::new(&native)).stdout;
        for body in emitted {
            std::fs::write(
                &source,
                format!(
                    "typedef int int4; typedef unsigned int uint4;\n{support}\n{body}\n{harness}"
                ),
            )
            .unwrap();
            for opt in ["-O0", "-O2"] {
                process::required_output(
                    Command::new(cc)
                        .args([opt, "-Werror=uninitialized", "-Werror=return-type"])
                        .arg(&source)
                        .arg("-o")
                        .arg(&rebuilt),
                );
                assert_eq!(
                    process::required_output(&mut Command::new(&rebuilt)).stdout,
                    expected,
                    "{cc} {opt}:\n{body}"
                );
            }
        }
    }
    for file in [driver, source, native, rebuilt] {
        let _ = std::fs::remove_file(file);
    }
}

#[test]
fn renamed_parameter_is_declared_and_read_under_the_same_name() {
    let image = image(ADD);
    let control = decompile(&image, PROTO, &[]);
    assert!(control.contains("return value + 1;"), "{control}");
    let renamed = decompile(&image, PROTO, &["name probe::value property"]);
    assert!(renamed.contains("int4 property)"), "{renamed}");
    assert!(renamed.contains("return property + 1;"), "{renamed}");
    assert!(!renamed.contains("value"), "{renamed}");
    semantic_oracle(&image, &[control, renamed], false);
    std::fs::remove_file(image).unwrap();
}

#[test]
fn renamed_register_keeps_its_incoming_value_before_a_conditional_overwrite() {
    for (bytes, prototype, msabi) in [(SYSV_CLEAR, PROTO, false), (MSABI_CLEAR, MS_PROTO, true)] {
        let image = image(bytes);
        let control = decompile(&image, prototype, &[]);
        let renamed = decompile(&image, prototype, &["name value property"]);
        assert!(renamed.contains("int4 property)"), "{renamed}");
        assert!(renamed.contains("if (property == 7)"), "{renamed}");
        assert!(renamed.contains("return property;"), "{renamed}");
        assert!(
            !renamed.contains("v1") && !renamed.contains("value"),
            "{renamed}"
        );
        semantic_oracle(&image, &[control, renamed], msabi);
        std::fs::remove_file(image).unwrap();
    }
}

#[test]
fn retyped_parameter_and_batched_aliases_reach_the_prototype() {
    let image = image(ADD);
    for assertions in [
        vec!["type value unsigned int"],
        vec!["name value property", "type property unsigned int"],
        vec!["type value unsigned int", "name value property"],
    ] {
        let code = decompile(&image, PROTO, &assertions);
        let name = if assertions.len() == 1 {
            "value"
        } else {
            "property"
        };
        assert!(code.contains(&format!("uint4 {name})")), "{code}");
        assert!(!code.contains("v1"), "{code}");
        semantic_oracle(&image, &[code], false);
    }
    std::fs::remove_file(image).unwrap();
}

#[test]
fn swapping_parameter_names_preserves_their_slots() {
    let image = image(ADD);
    for assertions in [
        ["name value unused", "name unused value"],
        ["name unused value", "name value unused"],
    ] {
        let code = decompile(&image, PROTO, &assertions);
        assert!(code.contains("probe(void *value,int4 unused)"), "{code}");
        assert!(code.contains("return unused + 1;"), "{code}");
        assert!(!code.contains("v1"), "{code}");
    }
    std::fs::remove_file(image).unwrap();
}

#[test]
fn a_stack_parameter_is_replayed_by_its_parameter_slot() {
    let image = image(&[0x8b, 0x44, 0x24, 0x08, 0xc3]);
    let prototype = "prototype probe int probe(int a,int b,int c,int d,int e,int f,int value)";
    let control = decompile(&image, prototype, &[]);
    assert!(control.contains("return value;"), "{control}");
    let renamed = decompile(&image, prototype, &["name value property"]);
    assert!(renamed.contains("int4 property)"), "{renamed}");
    assert!(renamed.contains("return property;"), "{renamed}");
    assert!(!renamed.contains("value"), "{renamed}");
    std::fs::remove_file(image).unwrap();
}

#[test]
fn missing_names_still_reject_and_leave_the_parameter_intact() {
    let image = image(ADD);
    for json in [false, true] {
        let (stdout, stderr, status) = run(&image, PROTO, &["name missing property"], json, false);
        assert_eq!(status, 1, "{stderr}\n{stdout}");
        if json {
            let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
            assert_eq!(doc["assertions"][1]["status"], "rejected");
            assert!(doc["functions"][0]["code"]
                .as_str()
                .unwrap()
                .contains("return value + 1;"));
        } else {
            assert!(stderr.contains("No symbol named: missing"), "{stderr}");
        }
    }
    std::fs::remove_file(image).unwrap();
}

const REGISTER_REUSE: &[u8] = &[
    0x48, 0x83, 0xec, 0x28, 0x83, 0xfa, 0x07, 0x74, 0x0c, 0x83, 0xfa, 0x0b, 0x75, 0x16, 0xba, 0x29,
    0, 0, 0, 0xeb, 0x05, 0xba, 0x2a, 0, 0, 0, 0xe8, 0x0c, 0, 0, 0, 0x48, 0x83, 0xc4, 0x28, 0xc3,
    0x31, 0xc0, 0x48, 0x83, 0xc4, 0x28, 0xc3, 0x89, 0xd0, 0xc3,
];

fn register_local(code: &str, register: &str) -> String {
    code.lines()
        .find_map(|line| {
            line.trim()
                .strip_suffix(&format!("; // {register}"))
                .and_then(|decl| decl.split_whitespace().last())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| panic!("no local at {register}:\n{code}"))
}

#[test]
fn later_register_alias_keeps_the_parameter_checks_and_call_value() {
    for msabi in [false, true] {
        let mut bytes = REGISTER_REUSE.to_vec();
        let (abi, register) = if msabi {
            ("MSABI ", "edx")
        } else {
            for offset in [5, 10] {
                bytes[offset] = 0xfe;
            }
            for offset in [14, 21] {
                bytes[offset] = 0xbe;
            }
            bytes[44] = 0xf0;
            ("", "esi")
        };
        let image = image_with_callee(&bytes, Some(43));
        let proto = format!("prototype probe int {abi}probe(void *popup,int property)");
        let callee = format!("prototype sink int {abi}sink(void *popup,int cleared)");
        let control = decompile(&image, &proto, &[&callee]);
        let local = register_local(&control, register);
        let directive = format!("name {local} cleared_property");
        let renamed = decompile(&image, &proto, &[&callee, &directive]);
        assert!(
            renamed.contains("probe(void *popup,int4 property)"),
            "{renamed}"
        );
        assert!(renamed.contains("if (property != 7)"), "{renamed}");
        assert!(renamed.contains("if (property != 0xb)"), "{renamed}");
        assert!(renamed.contains("cleared_property = 0x29;"), "{renamed}");
        assert!(renamed.contains("cleared_property = 0x2a;"), "{renamed}");
        assert!(
            renamed.contains("sink(popup,cleared_property)"),
            "{renamed}"
        );
        semantic_oracle_with_callee(
            &image,
            &[control, renamed],
            msabi,
            "int4 sink(void *popup,int4 cleared) { return cleared; }",
        );
        std::fs::remove_file(image).unwrap();
    }
}

#[test]
fn different_width_parameter_overlap_still_rejects() {
    let image = image_with_callee(REGISTER_REUSE, Some(43));
    let proto = "prototype probe int MSABI probe(void *popup,unsigned long long property)";
    let callee = "prototype sink int MSABI sink(void *popup,int cleared)";
    let control = decompile(&image, proto, &[callee]);
    let local = register_local(&control, "edx");
    let directive = format!("name {local} cleared_property");
    for (json, batch) in [(false, false), (true, false), (true, true)] {
        let (stdout, stderr, status) = run(&image, proto, &[callee, &directive], json, batch);
        assert_eq!(status, 1, "{stdout}\n{stderr}");
        if json {
            let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
            assert_eq!(doc["assertions"][2]["status"], "rejected");
            assert_eq!(
                doc["functions"][0]["code"].as_str().unwrap().trim(),
                control
            );
        } else {
            assert!(stderr.contains("overlaps a live parameter"), "{stderr}");
        }
    }
    std::fs::remove_file(image).unwrap();
}
