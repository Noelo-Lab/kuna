//! `passthrough` hands back the result of an ARM wrapper's call
//! (`push {r4,lr}; bl provider; pop {r4,pc}`) the way it already does for an
//! x86-64 `call provider; ret`, and keeps every refusal the rule has.
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
        match kind {
            "overwrite" => &[0xe92d4010, 0xeb000000, 0xe3a00007, 0xe8bd8010],
            "setarg" => &[0xe92d4010, 0xe3a00005, 0xeb000000, 0xe8bd8010],
            "bxlr" => &[0xe52de004, 0xeb000000, 0xe49de004, 0xe12fff1e],
            _ => &[0xe92d4010, 0xeb000000, 0xe8bd8010],
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
    let store = add("store", &[0xe3a01009, 0xe5801000, 0xe12fff1e]);
    let chain_void = add(
        "chain_void",
        &[0xe92d4010, 0xeb000000, 0xeb000000, 0xe8bd8010],
    );
    let chain_ret = add(
        "chain_ret",
        &[0xe92d4010, 0xeb000000, 0xeb000000, 0xe8bd8010],
    );
    let call = if kind == "setarg" {
        wrapper + 2
    } else {
        wrapper + 1
    };
    for (call, target) in [
        (call, if kind == "cycle" { middle } else { provider }),
        (middle + 1, wrapper),
        (outer + 1, middle),
        (consumer + 1, if kind == "chain" { outer } else { wrapper }),
        (void_wrapper + 1, sink),
        (chain_void + 1, provider),
        (chain_void + 2, store),
        (chain_ret + 1, provider),
        (chain_ret + 2, provider),
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

fn run(kind: &str, passthrough: bool, assertion: Option<&str>) -> String {
    let path = common::scratch_file("arm-wrapper-returns", "o");
    std::fs::write(&path, image(kind)).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args([
        "decompile-all",
        path.to_str().unwrap(),
        "--mode",
        "aggressive",
    ]);
    if !passthrough {
        command.args(["--option", "passthrough", "off"]);
    }
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
fn a_call_then_return_wrapper_hands_back_its_callee_result() {
    for kind in ["direct", "bxlr", "unused"] {
        let text = run(kind, true, None);
        assert!(
            function(&text, "wrapper").contains("unsigned int wrapper(unsigned int *a0)"),
            "{kind}: {text}"
        );
        assert!(
            function(&text, "wrapper").contains("return provider(a0);"),
            "{kind}: {text}"
        );
        assert!(
            function(&text, "consumer").contains("wrapper(a0)"),
            "{kind}: {text}"
        );
        let off = run(kind, false, None);
        assert!(
            function(&off, "wrapper").contains("void wrapper("),
            "{kind}: {off}"
        );
    }
    let text = run("direct", true, None);
    assert!(
        function(&text, "chain_void").contains("void chain_void("),
        "{text}"
    );
    assert!(
        function(&text, "chain_void").contains("store((unsigned int *)provider(a0));"),
        "{text}"
    );
    assert!(
        function(&text, "chain_ret").contains("unsigned int chain_ret("),
        "{text}"
    );
    assert!(
        function(&text, "chain_ret").contains("return provider("),
        "{text}"
    );
    assert!(
        function(&text, "chain_ret").contains("provider(a0)"),
        "{text}"
    );
    let text = run("chain", true, None);
    assert!(
        function(&text, "wrapper").contains("return provider(a0);"),
        "{text}"
    );
    assert!(
        function(&text, "middle").contains("return wrapper(a0);"),
        "{text}"
    );
    assert!(
        function(&text, "outer").contains("return middle(a0);"),
        "{text}"
    );
    assert!(function(&text, "consumer").contains("outer(a0)"), "{text}");
}

#[test]
fn clobbers_cycles_indirect_calls_and_void_callees_are_not_evidence() {
    for kind in ["sink", "cycle", "indirect"] {
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
    let text = run("setarg", true, None);
    assert!(
        function(&text, "wrapper").contains("void wrapper("),
        "{text}"
    );
    assert!(function(&text, "wrapper").contains("provider("), "{text}");
    assert!(!function(&text, "wrapper").contains("provider()"), "{text}");
    let text = run(
        "direct",
        true,
        Some("prototype wrapper void wrapper(unsigned int *p)"),
    );
    assert!(
        function(&text, "wrapper").contains("void wrapper("),
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
