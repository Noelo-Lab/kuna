//! A scalar local passed to a character-pointer callee printed earlier in a
//! callee-first run keeps its own type and width and is cast at the call.
mod common;
use common::process;

use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::path::PathBuf;
use std::process::Command;

/// One function: its name, its bytes, and the offset of the `call rel32`
/// opcode it makes to another function, if any.
type Func<'a> = (&'a str, Vec<u8>, Option<(usize, &'a str)>);

/// An x86-64 relocatable object with `funcs` laid out in order and every call
/// resolved to its callee.
fn object(funcs: &[Func]) -> PathBuf {
    let mut at = Vec::new();
    let mut code = Vec::new();
    for (_, bytes, _) in funcs {
        at.push(code.len());
        code.extend_from_slice(bytes);
    }
    for (i, (_, _, call)) in funcs.iter().enumerate() {
        if let Some((off, callee)) = call {
            let site = at[i] + off;
            let target = at[funcs.iter().position(|f| f.0 == *callee).unwrap()];
            let rel = target as i64 - (site + 5) as i64;
            code[site + 1..site + 5].copy_from_slice(&(rel as i32).to_le_bytes());
        }
    }
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(section, &code, 1);
    for (i, (name, bytes, _)) in funcs.iter().enumerate() {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value: at[i] as u64,
            size: bytes.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let path = common::scratch_file("pointer-call", "o");
    std::fs::write(&path, obj.write().unwrap()).unwrap();
    path
}

fn decompile(input: &PathBuf, extra: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile-all", input.to_str().unwrap()])
        .args(extra)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn compilers() -> Vec<&'static str> {
    let found: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!found.is_empty(), "the round trip requires a C compiler");
    found
}

/// Compile `printed` after `decls` and before `main`, with incompatible
/// pointers as errors, and run it.
fn round_trip(decls: &str, printed: &str, main: &str) {
    let src = common::scratch_file("pointer-call", "c");
    let exe = common::scratch_file("pointer-call", "exe");
    std::fs::write(&src, format!("{decls}\n{printed}\n{main}")).unwrap();
    for cc in compilers() {
        for level in ["-O0", "-O2"] {
            let compile = Command::new(cc)
                .args(["-std=c11", "-Werror=incompatible-pointer-types", level])
                .arg(&src)
                .arg("-o")
                .arg(&exe)
                .output()
                .unwrap();
            assert!(
                compile.status.success(),
                "{cc} {level}: {}\n{printed}",
                String::from_utf8_lossy(&compile.stderr)
            );
            assert!(
                Command::new(&exe).status().unwrap().success(),
                "{cc} {level}: {printed}"
            );
        }
    }
    for file in [src, exe] {
        std::fs::remove_file(file).unwrap();
    }
}

#[test]
fn a_byte_writing_callee_casts_its_callers_full_width_local_in_either_address_order() {
    // sink: xor byte [rdi],1; ret.  change: push rdi; mov rdi,rsp; call sink;
    // pop rax; ret.  Only the low byte of the pushed word changes.
    let sink: Func = ("sink", vec![0x80, 0x37, 1, 0xc3], None);
    let change: Func = (
        "change",
        vec![0x57, 0x48, 0x89, 0xe7, 0xe8, 0, 0, 0, 0, 0x58, 0xc3],
        Some((4, "sink")),
    );
    let main = r#"
int main(void) {
    unsigned long values[] = {0, 1, 0x123456789abcdef0UL, ~0UL, 0x8000000000000000UL};
    for (unsigned i = 0; i < sizeof(values)/sizeof(*values); ++i)
        if (change(values[i]) != (values[i] ^ 1)) return 1;
    return 0;
}
"#;
    for layout in [
        [sink.clone(), change.clone()],
        [change.clone(), sink.clone()],
    ] {
        let input = object(&layout);
        let assert = [
            "--assert",
            "prototype change unsigned long change(unsigned long)",
            "--assert-strict",
        ];
        let printed = decompile(&input, &assert);
        assert!(printed.contains("sink((unsigned char *)&"), "{printed}");
        assert!(printed.contains("unsigned long v"), "{printed}");
        round_trip("void sink(unsigned char *);", &printed, main);
        // Without the callee-first order no declaration is known at the call.
        let mut off = vec!["--option", "protoorder", "off"];
        off.extend_from_slice(&assert);
        let printed = decompile(&input, &off);
        assert!(printed.contains("sink(&v"), "{printed}");
        assert!(printed.contains("unsigned long v"), "{printed}");
        std::fs::remove_file(input).unwrap();
    }
}

#[test]
fn only_a_byte_of_another_signedness_or_width_decides_the_cast() {
    // byte_sink: a signed byte local passed to sink's `unsigned char *` -- the
    // same byte of the other signedness, so no cast.  word_wr: a 32-bit local
    // passed to wr, whose undefined byte prints as `char *` -- a cast.
    let byte_sink: Func = (
        "byte_sink",
        vec![
            0x55, 0x48, 0x89, 0xe5, 0x48, 0x83, 0xec, 0x10, 0x40, 0x88, 0x7d, 0xff, 0x48, 0x8d,
            0x7d, 0xff, 0xe8, 0, 0, 0, 0, 0x0f, 0xbe, 0x45, 0xff, 0xc9, 0xc3,
        ],
        Some((16, "sink")),
    );
    let word_wr: Func = (
        "word_wr",
        vec![
            0x55, 0x48, 0x89, 0xe5, 0x48, 0x83, 0xec, 0x10, 0x89, 0x7d, 0xfc, 0x48, 0x8d, 0x7d,
            0xfc, 0xe8, 0, 0, 0, 0, 0x8b, 0x45, 0xfc, 0xc9, 0xc3,
        ],
        Some((15, "wr")),
    );
    let sink: Func = ("sink", vec![0x80, 0x37, 1, 0xc3], None);
    let wr: Func = ("wr", vec![0xc6, 0x07, 7, 0xc3], None);
    let input = object(&[byte_sink, word_wr, sink, wr]);
    let printed = decompile(&input, &[]);
    assert!(
        printed.contains("void sink(unsigned char *a0)"),
        "{printed}"
    );
    assert!(printed.contains("void wr(char *a0)"), "{printed}");
    assert!(
        printed.contains("char v1;") && printed.contains("  sink(&v1);"),
        "{printed}"
    );
    assert!(
        printed.contains("unsigned int v1;") && printed.contains("  wr((char *)&v1);"),
        "{printed}"
    );
    round_trip(
        "void sink(unsigned char *);\nvoid wr(char *);",
        &printed,
        r#"
int main(void) {
    unsigned values[] = {0, 1, 0x7f, 0x80, 0xfe, 0xff, 0x12345678, ~0u};
    for (unsigned i = 0; i < sizeof(values)/sizeof(*values); ++i) {
        if (byte_sink((char)values[i]) != (char)((unsigned char)values[i] ^ 1)) return 1;
        if (word_wr(values[i]) != ((values[i] & ~0xffu) | 7)) return 2;
    }
    return 0;
}
"#,
    );
    std::fs::remove_file(input).unwrap();
}
