//! A caller that reads only the second register of a callee's 64-bit result
//! gets that result from the call. `bl full; uxtb r0,r1` overwrites `r0`
//! unread, and the read of `r1` printed as a local nothing assigns beside a
//! bare `full(a0);`, so the printed C computed garbage.
mod common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

/// The source every image implements, checked against the printed C under the
/// `s_` names.
const SOURCE: &str = r#"
static u64 s_full(unsigned a) { return ((u64)(a + 1) << 32) | (a * 3u); }
static unsigned s_hib(unsigned a) { return (unsigned char)(s_full(a) >> 32); }
static unsigned s_hiw(unsigned a) { return (unsigned)(s_full(a) >> 32); }
static unsigned s_hiplus(unsigned a) { return (unsigned)(s_full(a) >> 32) + 7; }
static unsigned s_lob(unsigned a) { return (unsigned)s_full(a) + 1; }
"#;

const MAIN: &str = r#"
static const unsigned V[] = {0, 1, 2, 7, 0x7e, 0x7f, 0xfe, 0xff, 0x7fff, 0x8000, 0xffff, 0x10000, 0x7fffffff,
                             0x80000000, 0x80000001, 0xfffffff8, 0xfffffffe, 0xffffffff, 0x12345678, 0xdeadbeef};
#define CHECK(got, want, mask) do { u64 g = (u64)(got) & (mask), w = (u64)(want) & (mask); if (g != w) { \
    printf("%s x=%#x: %llx, want %llx\n", #got, x, g, w); bad++; } } while (0)
int main(void) {
  int bad = 0;
  for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++) {
    unsigned x = V[i];
    CHECK(hib(x), s_hib(x), 0xff);
    CHECK(hiw(x), s_hiw(x), 0xffffffff);
    CHECK(hiplus(x), s_hiplus(x), 0xffffffff);
    CHECK(lob(x), s_lob(x), 0xffffffff);
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

/// `hib` is `push {r11,lr}; bl full; uxtb r0,r1; pop {r11,pc}`. `cw` reads
/// `r1` after `vw`, which writes it (`movw r1,#0x1000; str r0,[r1]`) but is
/// recovered `void`: the read is not a result.
fn arm_image() -> Vec<u8> {
    let tail = |op: u32| [0xe92d4800, 0, op, 0xe8bd8800];
    let funcs = [
        func("full", &[0xe2801001, 0xe0800080, 0xe12fff1e]),
        caller("hib", &tail(0xe6ef0071), 1, "full"),
        caller("hiw", &tail(0xe1a00001), 1, "full"),
        caller("hiplus", &tail(0xe2810007), 1, "full"),
        caller("lob", &tail(0xe2800001), 1, "full"),
        func("vw", &[0xe3011000, 0xe5810000, 0xe12fff1e]),
        caller("cw", &tail(0xe5910000), 1, "vw"),
    ];
    object(Architecture::Arm, 0x0500_0000, &funcs, |from, to| {
        (0xeb000000u32 | ((to as i64 - from as i64 - 8) >> 2) as u32 & 0xffffff).to_le_bytes().to_vec()
    })
}

/// `hib` is `jal full; nop; andi $2,$3,0xff`: `$3`, the high word, is
/// `full`'s. The loader lays `.text` out at 0x400000, which `jal` names.
fn mipsel_image() -> Vec<u8> {
    let tail = |op: u32| [0x27bdffe8, 0xafbf0014, 0, 0, op, 0x8fbf0014, 0x03e00008, 0x27bd0018];
    let funcs = [
        func("full", &[0x00040840, 0x00241021, 0x03e00008, 0x24830001]),
        caller("hib", &tail(0x306200ff), 2, "full"),
        caller("hiw", &tail(0x00601025), 2, "full"),
        caller("hiplus", &tail(0x24620007), 2, "full"),
        caller("lob", &tail(0x24420001), 2, "full"),
    ];
    object(Architecture::Mips, 0x7000_1005, &funcs, |_, to| {
        (0x0c000000u32 | ((0x400000 + to) >> 2) as u32 & 0x3ffffff).to_le_bytes().to_vec()
    })
}

/// `hib` is gcc's `call full; movzbl %dl,%eax`: the caller names only the
/// byte `DL` of the high word. `remx` reads `EDX` after `idv`, an `int a / b`
/// whose `idiv` leaves the remainder there: not part of what `idv` returns.
fn i386_image() -> Vec<u8> {
    let prologue = "83 ec 0c 8b 44 24 10 89 04 24 e8 00 00 00 00";
    let funcs = [
        hex("full", "8b 44 24 04 8d 50 01 8d 04 40 c3", None),
        hex("hib", &format!("{prologue} 0f b6 c2 83 c4 0c c3"), Some((10, "full"))),
        hex("hiw", &format!("{prologue} 89 d0 83 c4 0c c3"), Some((10, "full"))),
        hex("hiplus", &format!("{prologue} 8d 42 07 83 c4 0c c3"), Some((10, "full"))),
        hex("lob", &format!("{prologue} 83 c0 01 83 c4 0c c3"), Some((10, "full"))),
        hex("idv", "8b 44 24 04 99 f7 7c 24 08 c3", None),
        hex("remx", "ff 74 24 08 ff 74 24 08 e8 00 00 00 00 83 c4 08 89 d0 c3", Some((8, "idv"))),
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

fn body<'a>(arch: &str, text: &'a str, name: &str) -> &'a str {
    let start = text
        .find(&format!("// Function: {name} @"))
        .unwrap_or_else(|| panic!("{arch}: no function {name}:\n{text}"));
    let rest = &text[start..];
    &rest[..rest[1..].find("// Function:").map_or(rest.len(), |e| e + 1)]
}

const PRELUDE: &str = "#include <stdio.h>\n#include <stdbool.h>\ntypedef unsigned long long u64;\n\
                       static unsigned int dat_1000;\n\
                       #define CONCAT44(h, l) ((u64)(unsigned int)(h) << 32 | (unsigned int)(l))\n";

#[test]
fn a_callee_result_read_only_through_its_high_word_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let images = [("arm", arm_image()), ("mipsel", mipsel_image()), ("i386", i386_image())];
    for (arch, bytes) in images {
        let text = decompile(&format!("call-result-high-word-{arch}"), &bytes);
        for name in ["hib", "hiw", "hiplus"] {
            let f = body(arch, &text, name);
            assert!(f.contains("full(a0) >> 0x20"), "{arch}: {name} does not read the call's high word:\n{f}");
            assert!(!f.contains("; // "), "{arch}: {name} reads a register nothing assigns:\n{f}");
        }
        let control = match arch {
            "arm" => Some("cw"),
            "i386" => Some("remx"),
            _ => None,
        };
        if let Some(name) = control {
            let f = body(arch, &text, name);
            assert!(!f.contains(">> 0x20"), "{arch}: {name} takes a result its callee's recovery does not return:\n{f}");
        }
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let src = common::scratch_file(&format!("call-result-high-word-{arch}-{cc}{level}"), "c");
                let exe = src.with_extension("exe");
                std::fs::write(&src, format!("{PRELUDE}{text}\n{SOURCE}\n{MAIN}")).unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .args(common::CC_GCC15_DEMOTE)
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
