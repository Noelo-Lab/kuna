mod common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

fn image(kind: &str) -> Vec<u8> {
    let mut words = Vec::<u32>::new();
    let mut symbols = Vec::new();
    let mut add = |name: &str, code: &[u32]| {
        let at = words.len();
        words.extend_from_slice(code);
        symbols.push((name.to_string(), at, code.len()));
        at
    };
    let wrapper = add(
        "wrapper",
        if kind == "overwrite" {
            &[0xe92d4010, 0xeb000000, 0xe3a00007, 0xe8bd8010]
        } else {
            &[0xe92d4010, 0xeb000000, 0xe8bd8010]
        },
    );
    let provider = add(
        "provider",
        if kind == "sink" {
            &[0xe5801000, 0xe12fff1e]
        } else {
            &[0xe5900000, 0xe12fff1e]
        },
    );
    let middle = add("middle", &[0xe92d4010, 0xeb000000, 0xe8bd8010]);
    let outer = add("outer", &[0xe92d4010, 0xeb000000, 0xe8bd8010]);
    let consumer = add(
        "consumer",
        if kind == "unused" {
            &[0xe92d4010, 0xeb000000, 0xe3a00009, 0xe1a00000, 0xe8bd8010]
        } else {
            &[0xe92d4010, 0xeb000000, 0xe3500000, 0x03a00009, 0xe8bd8010]
        },
    );
    let void_wrapper = add("void_wrapper", &[0xe92d4010, 0xeb000000, 0xe8bd8010]);
    let sink = add("sink", &[0xe5801000, 0xe12fff1e]);
    for (call, target) in [
        (wrapper + 1, if kind == "cycle" { middle } else { provider }),
        (middle + 1, wrapper),
        (outer + 1, middle),
        (consumer + 1, if kind == "chain" { outer } else { wrapper }),
        (void_wrapper + 1, sink),
    ] {
        words[call] = 0xeb000000 | ((target as i32 - call as i32 - 2) as u32 & 0xffffff);
    }
    if kind == "indirect" {
        words[wrapper + 1] = 0xe12fff33;
    }
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    object.append_section_data(text, &bytes, 4);
    for (name, value, size) in symbols {
        object.add_symbol(Symbol {
            name: name.into_bytes(),
            value: value as u64 * 4,
            size: size as u64 * 4,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    object.write().unwrap()
}
fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let marker = format!("// Function: {name} @");
    text.split(&marker)
        .nth(1)
        .unwrap_or_else(|| panic!("{text}"))
        .split("// Function:")
        .next()
        .unwrap()
}
fn run(kind: &str, on: bool, assertion: Option<&str>) -> String {
    let path = common::scratch_file("arm-wrapper-returns", "o");
    std::fs::write(&path, image(kind)).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args([
        "decompile-all",
        path.to_str().unwrap(),
        "--mode",
        "aggressive",
        "--option",
        "wrapperreturn",
        if on { "on" } else { "off" },
    ]);
    if let Some(assertion) = assertion {
        command.args(["--assert", assertion, "--assert-strict"]);
    }
    let output = command.output().unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        function(&text, "void_wrapper").contains("void void_wrapper("),
        "{text}"
    );
    text
}
#[test]
fn consumed_results_have_consistent_contracts_through_a_chain() {
    for kind in ["direct", "chain"] {
        let text = run(kind, true, None);
        assert!(
            function(&text, "wrapper").contains("return provider(a0);"),
            "{text}"
        );
        if kind == "chain" {
            assert!(
                function(&text, "middle").contains("return wrapper(a0);"),
                "{text}"
            );
            assert!(
                function(&text, "outer").contains("return middle(a0);"),
                "{text}"
            );
            assert!(function(&text, "consumer").contains("outer(a0)"), "{text}");
        } else {
            assert!(
                function(&text, "consumer").contains("wrapper(a0)"),
                "{text}"
            );
        }
        assert!(
            !function(&text, "consumer").contains("void consumer"),
            "{text}"
        );
        let off = run(kind, false, None);
        assert!(function(&off, "wrapper").contains("void wrapper("), "{off}");
    }
}
#[test]
fn silence_clobbers_cycles_indirect_calls_and_explicit_void_are_not_evidence() {
    for kind in ["unused", "sink", "cycle", "indirect"] {
        let text = run(kind, true, None);
        assert!(
            function(&text, "wrapper").contains("void wrapper("),
            "{kind}: {text}"
        );
    }
    let text = run("overwrite", true, None);
    assert!(function(&text, "wrapper").contains("return 7;"), "{text}");
    assert!(
        !function(&text, "wrapper").contains("return provider"),
        "{text}"
    );
    let text = run(
        "direct",
        true,
        Some("prototype wrapper void wrapper(unsigned int *p)"),
    );
    assert!(
        function(&text, "wrapper").contains("void wrapper("),
        "{text}"
    );
}

#[test]
fn declared_provider_results_are_evidence_but_declared_void_is_not() {
    let text = run(
        "direct",
        true,
        Some("prototype provider unsigned int provider(unsigned int *p)"),
    );
    assert!(
        function(&text, "wrapper").contains("return provider(a0);"),
        "{text}"
    );
    assert!(
        function(&text, "consumer").contains("wrapper(a0)"),
        "{text}"
    );
    let text = run(
        "direct",
        true,
        Some("prototype provider void provider(unsigned int *p)"),
    );
    assert!(
        function(&text, "wrapper").contains("void wrapper("),
        "{text}"
    );
}
