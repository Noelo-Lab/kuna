//! A caller that hands back a callee's 64-bit result with only the high word
//! changed returns all eight bytes. `bl full; orr r1,r1,#255; bx lr` leaves
//! `r0` as `full` returned it, and the function printed as `void`, so a
//! caller of the printed C read nothing at all.
mod common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

/// The source of every image, built with `clang --target=<triple> -O2
/// -fno-pic` and checked against the printed C under the `s_` names.
const SOURCE: &str = r#"
static u64 s_full(unsigned a) { return ((u64)(a + 1) << 32) | (a * 3u); }
static int s_g32s(int a) { return a * 7 - 100; }
static u64 s_full_or(unsigned a) { return s_full(a) | 0xff00000000ULL; }
static u64 s_full_add(unsigned a) { return s_full(a) + (1ULL << 32); }
static u64 s_hi_const(unsigned a) { return (7ULL << 32) | (unsigned)s_g32s(a); }
static s64 s_sext(int a) { return s_g32s(a); }
"#;

const MAIN: &str = r#"
static const unsigned V[] = {0, 1, 2, 7, 0x7fff, 0x8000, 0xffff, 0x10000, 0x7fffffff, 0x80000000,
                             0x80000001, 0xfffffff8, 0xfffffffe, 0xffffffff, 0x12345678, 0xdeadbeef};
#define CHECK(got, want) do { u64 g = (got), w = (want); if (g != w) { \
    printf("%s x=%#x: %016llx, want %016llx\n", #got, x, g, w); bad++; } } while (0)
int main(void) {
  int bad = 0;
  for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++) {
    unsigned x = V[i];
    CHECK(full_or(x), s_full_or(x));
    CHECK(full_add(x), s_full_add(x));
    CHECK(hi_const(x), s_hi_const(x));
    CHECK(sext(x), s_sext(x));
  }
  return bad != 0;
}
"#;

/// One function: its bytes, and the offset and target of the call it makes.
struct Func {
    name: &'static str,
    code: Vec<u8>,
    call: Option<(usize, &'static str)>,
}

fn words(code: &[u32]) -> Vec<u8> {
    code.iter().flat_map(|w| w.to_le_bytes()).collect()
}

fn func(name: &'static str, code: &[u32]) -> Func {
    Func { name, code: words(code), call: None }
}

fn caller(name: &'static str, code: &[u32], word: usize, target: &'static str) -> Func {
    Func { name, code: words(code), call: Some((word * 4, target)) }
}

fn hex(name: &'static str, code: &str, call: Option<(usize, &'static str)>) -> Func {
    let code = code.split_whitespace().map(|b| u8::from_str_radix(b, 16).unwrap()).collect();
    Func { name, code, call }
}

/// A little-endian relocatable object whose `.text` is `funcs` back to back,
/// each call encoded by `call(from, to)`.
fn object(arch: Architecture, e_flags: u32, funcs: &[Func], call: impl Fn(u64, u64) -> Vec<u8>) -> Vec<u8> {
    let mut text = Vec::new();
    let mut at = Vec::new();
    for f in funcs {
        at.push(text.len() as u64);
        text.extend_from_slice(&f.code);
    }
    let start = |name| at[funcs.iter().position(|f| f.name == name).unwrap()];
    for (f, &base) in funcs.iter().zip(&at) {
        if let Some((off, target)) = f.call {
            let from = base + off as u64;
            let bytes = call(from, start(target));
            text[from as usize..from as usize + bytes.len()].copy_from_slice(&bytes);
        }
    }
    let mut object = Object::new(BinaryFormat::Elf, arch, Endianness::Little);
    object.flags = FileFlags::Elf { os_abi: 0, abi_version: 0, e_flags };
    let section = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    object.append_section_data(section, &text, 16);
    for (f, &value) in funcs.iter().zip(&at) {
        object.add_symbol(Symbol {
            name: f.name.as_bytes().to_vec(),
            value,
            size: f.code.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    object.write().unwrap()
}

/// `full_or` is `push {r11,lr}; bl full; orr r1,r1,#255; pop {r11,lr}; bx lr`.
fn arm_image() -> Vec<u8> {
    let funcs = [
        func("full", &[0xe0802080, 0xe2801001, 0xe1a00002, 0xe12fff1e]),
        func("g32s", &[0xe0600180, 0xe2400064, 0xe12fff1e]),
        caller("full_or", &[0xe92d4800, 0, 0xe38110ff, 0xe8bd4800, 0xe12fff1e], 1, "full"),
        caller("full_add", &[0xe92d4800, 0, 0xe2811001, 0xe8bd4800, 0xe12fff1e], 1, "full"),
        caller("hi_const", &[0xe92d4800, 0, 0xe3a01007, 0xe8bd4800, 0xe12fff1e], 1, "g32s"),
        caller("sext", &[0xe92d4800, 0, 0xe1a01fc0, 0xe8bd4800, 0xe12fff1e], 1, "g32s"),
    ];
    object(Architecture::Arm, 0x0500_0000, &funcs, |from, to| {
        (0xeb000000u32 | ((to as i64 - from as i64 - 8) >> 2) as u32 & 0xffffff).to_le_bytes().to_vec()
    })
}

/// `full_or` is `jal full; nop; ori $3,$3,0xff`: `$2`, the low word, is
/// `full`'s. The loader lays `.text` out at 0x400000, which `jal` names.
fn mipsel_image() -> Vec<u8> {
    let tail = |op: u32| [0x27bdffe8, 0xafbf0014, 0, 0, op, 0x8fbf0014, 0x03e00008, 0x27bd0018];
    let funcs = [
        func("full", &[0x00040840, 0x00241021, 0x03e00008, 0x24830001]),
        func("g32s", &[0x000408c0, 0x00240823, 0x03e00008, 0x2422ff9c]),
        caller("full_or", &tail(0x346300ff), 2, "full"),
        caller("full_add", &tail(0x24630001), 2, "full"),
        caller("hi_const", &tail(0x24030007), 2, "g32s"),
        caller("sext", &tail(0x00021fc3), 2, "g32s"),
    ];
    object(Architecture::Mips, 0x7000_1005, &funcs, |_, to| {
        (0x0c000000u32 | ((0x400000 + to) >> 2) as u32 & 0x3ffffff).to_le_bytes().to_vec()
    })
}

/// `full_or` is `call full; or $0xff,%edx; ret`. `keepz` is what
/// `-fzero-call-used-regs` makes of `int keepz(int a) { return g32s(a); }`:
/// `call g32s; xor %edx,%edx; ret` returns four bytes, not eight.
fn i386_image() -> Vec<u8> {
    let prologue = "83 ec 0c 8b 44 24 10 89 04 24 e8 00 00 00 00";
    let funcs = [
        hex("full", "8b 44 24 04 8d 50 01 8d 04 40 c3", None),
        hex("g32s", "8b 4c 24 04 8d 04 cd 00 00 00 00 29 c8 83 c0 9c c3", None),
        hex("full_or", &format!("{prologue} 81 ca ff 00 00 00 83 c4 0c c3"), Some((10, "full"))),
        hex("full_add", &format!("{prologue} 83 c2 01 83 c4 0c c3"), Some((10, "full"))),
        hex("hi_const", &format!("{prologue} ba 07 00 00 00 83 c4 0c c3"), Some((10, "g32s"))),
        hex("sext", &format!("{prologue} 89 c2 c1 fa 1f 83 c4 0c c3"), Some((10, "g32s"))),
        hex("keepz", &format!("{prologue} 31 d2 83 c4 0c c3"), Some((10, "g32s"))),
    ];
    object(Architecture::I386, 0, &funcs, |from, to| {
        let mut bytes = vec![0xe8];
        bytes.extend_from_slice(&((to as i64 - from as i64 - 5) as i32).to_le_bytes());
        bytes
    })
}

fn decompile(stem: &str, bytes: &[u8]) -> String {
    let path = common::scratch_file(stem, "o");
    std::fs::write(&path, bytes).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile-all", path.to_str().unwrap()])
        .output()
        .unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "{text}\n{}", String::from_utf8_lossy(&output.stderr));
    text
}

fn declaration<'a>(arch: &str, text: &'a str, name: &str) -> &'a str {
    text.lines()
        .find(|l| l.contains(&format!(" {name}(")) && !l.starts_with(' '))
        .unwrap_or_else(|| panic!("{arch}: no declaration of {name}:\n{text}"))
}

const PRELUDE: &str = "#include <stdio.h>\n#include <stdbool.h>\ntypedef unsigned long long u64;\n\
                       typedef long long s64;\n\
                       #define CONCAT44(h, l) ((u64)(unsigned int)(h) << 32 | (unsigned int)(l))\n";

#[test]
fn a_callee_result_with_a_changed_high_word_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let images = [("arm", arm_image()), ("mipsel", mipsel_image()), ("i386", i386_image())];
    for (arch, bytes) in images {
        let text = decompile(&format!("call-result-pair-{arch}"), &bytes);
        for name in ["full_or", "full_add", "hi_const", "sext"] {
            assert!(
                !declaration(arch, &text, name).starts_with("void "),
                "{arch}: {name} lost its return value:\n{text}"
            );
        }
        if arch == "i386" {
            let keepz = declaration(arch, &text, "keepz");
            assert!(!keepz.contains("long long"), "{arch}: a hardening zero widened keepz:\n{text}");
        }
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let src = common::scratch_file(&format!("call-result-pair-{arch}-{cc}{level}"), "c");
                let exe = src.with_extension("exe");
                std::fs::write(&src, format!("{PRELUDE}{text}\n{SOURCE}\n{MAIN}")).unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} {level} rejected the printed C ({arch}):\n{}\n{text}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = Command::new(&exe).output().expect("run the round trip");
                assert!(
                    run.status.success(),
                    "{arch} printed and built by {cc} {level} computes a different value:\n{}\n{text}",
                    String::from_utf8_lossy(&run.stdout)
                );
            }
        }
    }
}
