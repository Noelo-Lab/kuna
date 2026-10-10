//! `passthrough` hands back the result of an ARM, Thumb or MIPS wrapper's call
//! (`push {r4,lr}; bl provider; pop {r4,pc}`, `jal provider; ...; jr ra`) the
//! way it already does for an x86-64 `call provider; ret`, and keeps every
//! refusal the rule has.
use crate::common;
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
    let text = decompile(&image(kind), "o", passthrough, assertion);
    assert!(
        function(&text, "void_wrapper").contains("void void_wrapper("),
        "{text}"
    );
    text
}

fn decompile(bytes: &[u8], ext: &str, passthrough: bool, assertion: Option<&str>) -> String {
    let path = common::scratch_file("arm-wrapper-returns", ext);
    std::fs::write(&path, bytes).unwrap();
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
    text
}

/// A Thumb object: every function's symbol carries the Thumb bit.
fn thumb_image() -> Vec<u8> {
    let mut code = Vec::<u16>::new();
    let mut symbols = Vec::new();
    let mut calls = Vec::new();
    let mut add = |name: &str, body: &[&str]| {
        let at = code.len();
        for op in body {
            match *op {
                "bl" => {
                    calls.push((code.len(), body.len()));
                    code.extend_from_slice(&[0xf000, 0xf800]);
                }
                op => code.push(u16::from_str_radix(op, 16).unwrap()),
            }
        }
        symbols.push((name.to_string(), at, code.len() - at));
    };
    add("provider", &["6800", "4770"]);
    add("store", &["2109", "6001", "4770"]);
    add("t_pop", &["b510", "bl", "bd10"]);
    add("t_bxlr", &["b510", "bl", "e8bd", "4010", "4770"]);
    add("t_chain", &["b510", "bl", "bl", "bd10"]);
    add("t_store", &["b510", "bl", "bl", "bd10"]);
    add("t_setarg", &["b510", "2005", "bl", "bd10"]);
    let at = |name: &str| symbols.iter().find(|s| s.0 == name).unwrap().1;
    let targets = [
        "provider", "provider", "provider", "provider", "provider", "store", "provider",
    ];
    for (&(call, _), target) in calls.iter().zip(targets) {
        let offset = (at(target) as i64 - call as i64 - 2) * 2;
        code[call] = 0xf000 | ((offset >> 12) as u16 & 0x7ff);
        code[call + 1] = 0xf800 | ((offset >> 1) as u16 & 0x7ff);
    }
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let bytes: Vec<_> = code.into_iter().flat_map(u16::to_le_bytes).collect();
    object.append_section_data(text, &bytes, 4);
    for (name, value, size) in symbols {
        object.add_symbol(Symbol {
            name: name.into_bytes(),
            value: value as u64 * 2 + 1,
            size: size as u64 * 2,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    object.write().unwrap()
}

/// A linked MIPS32 executable at 0x400000: `jal` is absolute, and kuna does
/// not apply a MIPS object's relocations.
fn mips_image(big: bool) -> Vec<u8> {
    const BASE: u32 = 0x400000;
    let frame = |call: u32, after: &[u32]| {
        let mut body = vec![0x27bdffe8, 0xafbf0014, call, 0, 0x8fbf0014];
        body.extend_from_slice(after);
        body.extend_from_slice(&[0x03e00008, 0x27bd0018]);
        body
    };
    let bodies: Vec<(&str, Vec<u32>)> = vec![
        ("provider", vec![0x8c820000, 0x03e00008, 0]),
        ("wrapper", frame(0x0c000000, &[])),
        ("w_after", frame(0x0c000000, &[0x24020007])),
        ("w_jalr", frame(0x00a0f809, &[])),
        ("consumer", {
            let mut b = frame(0x0c000000, &[]);
            b.insert(4, 0x24420001);
            b
        }),
    ];
    let mut words = Vec::new();
    let mut symbols = Vec::new();
    for (name, body) in &bodies {
        symbols.push((*name, words.len() as u32 * 4, body.len() as u32 * 4));
        words.extend_from_slice(body);
    }
    let jal = |target: u32| 0x0c000000 | ((BASE + target) >> 2 & 0x3ffffff);
    for (i, w) in words.iter_mut().enumerate() {
        if *w == 0x0c000000 {
            let caller = symbols
                .iter()
                .rev()
                .find(|s| s.1 <= i as u32 * 4)
                .unwrap()
                .0;
            let target = if caller == "consumer" {
                "wrapper"
            } else {
                "provider"
            };
            *w = jal(symbols.iter().find(|s| s.0 == target).unwrap().1);
        }
    }
    let h = |v: u16| {
        if big {
            v.to_be_bytes()
        } else {
            v.to_le_bytes()
        }
    };
    let w = |v: u32| {
        if big {
            v.to_be_bytes()
        } else {
            v.to_le_bytes()
        }
    };
    let code: Vec<u8> = words.iter().flat_map(|&x| w(x)).collect();
    let mut strtab = vec![0u8];
    let mut symtab = vec![0u8; 16];
    for (name, value, size) in &symbols {
        symtab.extend(w(strtab.len() as u32));
        symtab.extend(w(BASE + value));
        symtab.extend(w(*size));
        symtab.extend([0x12, 0]);
        symtab.extend(h(1));
        strtab.extend(name.bytes().chain([0]));
    }
    let shstr = b"\0.text\0.symtab\0.strtab\0.shstrtab\0";
    let t_off = 0x1000u32;
    let s_off = t_off + code.len() as u32;
    let st_off = s_off + symtab.len() as u32;
    let sh_str = st_off + strtab.len() as u32;
    let shoff = (sh_str + shstr.len() as u32 + 7) & !7;
    let mut out = vec![0u8; shoff as usize];
    let mut ehdr = vec![0x7f, b'E', b'L', b'F', 1, if big { 2 } else { 1 }, 1];
    ehdr.resize(16, 0);
    ehdr.extend(h(2));
    ehdr.extend(h(8));
    for v in [1, BASE, 52, shoff, 0x5000_1000] {
        ehdr.extend(w(v));
    }
    for v in [52, 32, 1, 40, 5, 4] {
        ehdr.extend(h(v));
    }
    out[..52].copy_from_slice(&ehdr);
    let phdr: Vec<u8> = [
        1,
        t_off,
        BASE,
        BASE,
        code.len() as u32,
        code.len() as u32,
        5,
        0x1000,
    ]
    .into_iter()
    .flat_map(w)
    .collect();
    out[52..84].copy_from_slice(&phdr);
    out[t_off as usize..s_off as usize].copy_from_slice(&code);
    out[s_off as usize..st_off as usize].copy_from_slice(&symtab);
    out[st_off as usize..sh_str as usize].copy_from_slice(&strtab);
    out[sh_str as usize..sh_str as usize + shstr.len()].copy_from_slice(shstr);
    let sections = [
        [0; 10],
        [1, 1, 6, BASE, t_off, code.len() as u32, 0, 0, 16, 0],
        [7, 2, 0, 0, s_off, symtab.len() as u32, 3, 1, 4, 16],
        [15, 3, 0, 0, st_off, strtab.len() as u32, 0, 0, 1, 0],
        [23, 3, 0, 0, sh_str, shstr.len() as u32, 0, 0, 1, 0],
    ];
    for section in sections {
        out.extend(section.into_iter().flat_map(w));
    }
    out
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
        // `consumer` reads r0 after the call, so decompile-all's `voidret` redo
        // returns it without passthrough; only an unread result stays void.
        let off = run(kind, false, None);
        let want = if kind == "unused" { "void wrapper(" } else { "return provider(a0);" };
        assert!(function(&off, "wrapper").contains(want), "{kind}: {off}");
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
    for kind in ["sink", "cycle"] {
        let text = run(kind, true, None);
        assert!(
            function(&text, "wrapper").contains("void wrapper("),
            "{kind}: {text}"
        );
    }
    // Neither is passthrough's evidence, but `consumer` reads r0 after the call.
    let text = run("indirect", true, None);
    assert!(function(&text, "wrapper").contains("return (*a3)();"), "{text}");
    let text = run("overwrite", true, None);
    assert!(function(&text, "wrapper").contains("return 7;"), "{text}");
    assert!(
        !function(&text, "wrapper").contains("return provider"),
        "{text}"
    );
    let text = run("setarg", true, None);
    assert!(
        function(&text, "wrapper").contains("return provider((unsigned int *)0x5);"),
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

#[test]
fn thumb_wrappers_hand_back_their_callee_result() {
    let text = decompile(&thumb_image(), "o", true, None);
    for name in ["t_pop", "t_bxlr"] {
        assert!(
            function(&text, name).contains(&format!("unsigned int {name}(unsigned int *a0)")),
            "{text}"
        );
        assert!(
            function(&text, name).contains("return provider(a0);"),
            "{text}"
        );
    }
    assert!(
        function(&text, "t_chain").contains("return provider("),
        "{text}"
    );
    assert!(
        function(&text, "t_chain").contains("provider(a0)"),
        "{text}"
    );
    assert!(
        function(&text, "t_store").contains("store((unsigned int *)provider(a0));"),
        "{text}"
    );
    assert!(
        function(&text, "t_setarg").contains("void t_setarg("),
        "{text}"
    );
    assert!(
        !function(&text, "t_setarg").contains("provider()"),
        "{text}"
    );
    let off = decompile(&thumb_image(), "o", false, None);
    assert!(function(&off, "t_pop").contains("void t_pop("), "{off}");
}

#[test]
fn mips_wrappers_hand_back_their_callee_result() {
    for big in [false, true] {
        let text = decompile(&mips_image(big), "elf", true, None);
        assert!(
            function(&text, "wrapper").contains("unsigned int wrapper(unsigned int *a0)"),
            "{text}"
        );
        assert!(
            function(&text, "wrapper").contains("return provider(a0);"),
            "{text}"
        );
        assert!(function(&text, "w_after").contains("return 7;"), "{text}");
        assert!(function(&text, "w_jalr").contains("void w_jalr("), "{text}");
        assert!(
            function(&text, "consumer").contains("wrapper(a0)"),
            "{text}"
        );
        let off = decompile(&mips_image(big), "elf", false, None);
        assert!(function(&off, "wrapper").contains("return provider("), "{off}");
    }
}
