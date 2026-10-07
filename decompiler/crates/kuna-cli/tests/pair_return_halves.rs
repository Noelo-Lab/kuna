//! A 64-bit return on a 32-bit target whose high word is computed from the
//! low word (`add r0,r0,#3; asr r1,r0,#31`), or whose low word is the argument
//! left in its register (`mov r1,#0; bx lr`), returns all eight bytes. Both
//! printed as `void f(void)`, so a caller of the printed C read nothing.
use crate::common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

/// The source of every image, built with `clang --target=<triple> -O2` (and
/// gcc -m32 -O2 for the `_g` functions), checked against the printed C under
/// the `s_` names.
const SOURCE: &str = r#"
static s64 s_sx(int a) { return a + 3; }
static u64 s_zx(unsigned a) { return a; }
static s64 s_sxa(int a) { return a; }
static u64 s_kc(unsigned a) { return a | (5ULL << 32); }
static u64 s_dup(unsigned a) { return a * 0x100000001ULL; }
static u64 s_dupc(unsigned a) { unsigned x = a + 1; return ((u64)x << 32) | x; }
static s64 s_sxsel(int a, int b) { return a > b ? a : b; }
static s64 s_sxsub(int a, int b) { return a - b; }
"#;

/// Each checked function, the source function it must agree with, and how
/// many arguments it takes.
const CHECKS: &[(&str, &str, usize)] = &[
    ("sx", "s_sx", 1),
    ("zx", "s_zx", 1),
    ("sxa", "s_sxa", 1),
    ("kc", "s_kc", 1),
    ("kc0", "s_kc", 1),
    ("dup", "s_dup", 1),
    ("dupc", "s_dupc", 1),
    ("sxsel", "s_sxsel", 2),
    ("sxsub", "s_sxsub", 2),
    ("sx_g", "s_sx", 1),
    ("sxsel_g", "s_sxsel", 2),
];

struct Func {
    name: &'static str,
    code: Vec<u8>,
}

fn words(name: &'static str, code: &[u32]) -> Func {
    Func { name, code: code.iter().flat_map(|w| w.to_le_bytes()).collect() }
}

fn hex(name: &'static str, code: &str) -> Func {
    Func { name, code: code.split_whitespace().map(|b| u8::from_str_radix(b, 16).unwrap()).collect() }
}

/// A little-endian relocatable object whose `.text` is `funcs` back to back.
fn object(arch: Architecture, e_flags: u32, funcs: &[Func]) -> Vec<u8> {
    let mut object = Object::new(BinaryFormat::Elf, arch, Endianness::Little);
    object.flags = FileFlags::Elf { os_abi: 0, abi_version: 0, e_flags };
    let section = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let mut text = Vec::new();
    let mut at = Vec::new();
    for f in funcs {
        at.push(text.len() as u64);
        text.extend_from_slice(&f.code);
    }
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

/// `sx` is `add r0,r0,#3; asr r1,r0,#31; bx lr`, `zx` is `mov r1,#0; bx lr`,
/// and `kc0` is clang -O0's `kc`, which reloads its argument from its spill.
fn arm_image() -> Vec<u8> {
    let funcs = [
        words("sx", &[0xe2800003, 0xe1a01fc0, 0xe12fff1e]),
        words("zx", &[0xe3a01000, 0xe12fff1e]),
        words("sxa", &[0xe1a01fc0, 0xe12fff1e]),
        words("kc", &[0xe3a01005, 0xe12fff1e]),
        words("kc0", &[0xe24dd004, 0xe58d0000, 0xe59d0000, 0xe3a01005, 0xe28dd004, 0xe12fff1e]),
        words("dup", &[0xe1a01000, 0xe12fff1e]),
        words("dupc", &[0xe2800001, 0xe1a01000, 0xe12fff1e]),
        words("sxsel", &[0xe1500001, 0xd1a00001, 0xe1a01fc0, 0xe12fff1e]),
        words("sxsub", &[0xe0400001, 0xe1a01fc0, 0xe12fff1e]),
    ];
    object(Architecture::Arm, 0x0500_0000, &funcs)
}

/// `sx` is `addiu $2,$4,3; jr $ra; sra $3,$2,31`: the high word is set in the
/// delay slot from the low word.
fn mipsel_image() -> Vec<u8> {
    let funcs = [
        words("sx", &[0x24820003, 0x03e00008, 0x00021fc3]),
        words("zx", &[0x00801025, 0x03e00008, 0x24030000]),
        words("sxa", &[0x00041fc3, 0x03e00008, 0x00801025]),
        words("kc", &[0x00801025, 0x03e00008, 0x24030005]),
        words("dup", &[0x00801025, 0x03e00008, 0x00801825]),
        words("dupc", &[0x24820001, 0x03e00008, 0x00401825]),
        words("sxsel", &[0x00a4082a, 0x0081280b, 0x00051fc3, 0x03e00008, 0x00a01025]),
        words("sxsub", &[0x00851023, 0x03e00008, 0x00021fc3]),
    ];
    object(Architecture::Mips, 0x7000_1005, &funcs)
}

/// clang's `sx` is `mov 4(%esp),%eax; add $3,%eax; mov %eax,%edx; sar
/// $31,%edx; ret`; gcc's `sx_g` ends `cltd; ret`.
fn i386_image() -> Vec<u8> {
    let funcs = [
        hex("sx", "8b 44 24 04 83 c0 03 89 c2 c1 fa 1f c3"),
        hex("zx", "8b 44 24 04 31 d2 c3"),
        hex("sxa", "8b 44 24 04 89 c2 c1 fa 1f c3"),
        hex("kc", "8b 44 24 04 ba 05 00 00 00 c3"),
        hex("dup", "8b 44 24 04 89 c2 c3"),
        hex("dupc", "8b 44 24 04 83 c0 01 89 c2 c3"),
        hex("sxsel", "8b 44 24 08 8b 4c 24 04 39 c1 0f 4f c1 89 c2 c1 fa 1f c3"),
        hex("sxsub", "8b 44 24 04 2b 44 24 08 89 c2 c1 fa 1f c3"),
        hex("sx_g", "8b 44 24 04 83 c0 03 99 c3"),
        hex("sxsel_g", "8b 54 24 04 8b 44 24 08 39 d0 0f 4c c2 99 c3"),
    ];
    object(Architecture::I386, 0, &funcs)
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

fn declaration<'a>(arch: &str, text: &'a str, name: &str) -> Option<&'a str> {
    let found = text.lines().find(|l| l.contains(&format!(" {name}(")) && !l.starts_with(' '));
    assert!(found.is_some() || !text.contains(&format!("Function: {name} ")), "{arch}: no declaration of {name}:\n{text}");
    found
}

const PRELUDE: &str = "#include <stdio.h>\n#include <stdbool.h>\ntypedef unsigned long long u64;\n\
                       typedef long long s64;\n\
                       #define CONCAT44(h, l) ((u64)(unsigned int)(h) << 32 | (unsigned int)(l))\n";

/// The checks for the functions `text` declares, over every pair of `V`.
fn main_for(text: &str) -> String {
    let mut body = String::new();
    for &(name, want, arity) in CHECKS {
        if !text.contains(&format!("Function: {name} ")) {
            continue;
        }
        let args = if arity == 1 { "x" } else { "x, y" };
        body.push_str(&format!("      CHECK({name}({args}), {want}({args}));\n"));
    }
    format!(
        r#"
static const unsigned V[] = {{0, 1, 2, 7, 0x7fff, 0x8000, 0x7ffffffd, 0x7fffffff, 0x80000000,
                             0x80000001, 0xfffffff8, 0xfffffffe, 0xffffffff, 0x12345678, 0xdeadbeef}};
#define CHECK(got, want) do {{ u64 g = (got), w = (want); if (g != w) {{ \
    printf("%s x=%#x y=%#x: %016llx, want %016llx\n", #got, x, y, g, w); bad++; }} }} while (0)
int main(void) {{
  int bad = 0;
  for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++)
    for (unsigned j = 0; j < sizeof V / sizeof V[0]; j++) {{
      unsigned x = V[i], y = V[j];
{body}    }}
  return bad != 0;
}}
"#
    )
}

#[test]
fn a_high_word_built_from_the_low_word_or_beside_the_argument_round_trips() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let images = [("arm", arm_image()), ("mipsel", mipsel_image()), ("i386", i386_image())];
    for (arch, bytes) in images {
        let text = decompile(&format!("pair-return-halves-{arch}"), &bytes);
        for &(name, _, _) in CHECKS {
            if let Some(decl) = declaration(arch, &text, name) {
                assert!(!decl.starts_with("void "), "{arch}: {name} lost its return value:\n{text}");
            }
        }
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let src = common::scratch_file(&format!("pair-return-halves-{arch}-{cc}{level}"), "c");
                let exe = src.with_extension("exe");
                std::fs::write(&src, format!("{PRELUDE}{text}\n{SOURCE}\n{}", main_for(&text))).unwrap();
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

/// A word the function returns in its first register alone, or a high
/// register that only feeds the low one, is not widened.
#[test]
fn a_word_return_and_a_high_register_feeding_the_low_one_stay_words() {
    let funcs = [
        words("div7", &[0xe1a01140, 0xe0810fa0, 0xe12fff1e]),
        words("plain", &[0xe0800001, 0xe12fff1e]),
        words("scrub", &[0xe3a02001, 0xe5812000, 0xe3a01000, 0xe3a02000, 0xe12fff1e]),
    ];
    let text = decompile("pair-return-halves-words", &object(Architecture::Arm, 0x0500_0000, &funcs));
    for name in ["div7", "plain"] {
        let decl = declaration("arm", &text, name).unwrap();
        assert!(decl.starts_with("int "), "arm: {name} is a word:\n{text}");
    }
    let scrub = declaration("arm", &text, "scrub").unwrap();
    assert!(scrub.starts_with("void "), "arm: the zero in an r1 the body used is a scrub:\n{text}");
}
