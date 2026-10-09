//! A 64-bit return that a 32-bit target builds from a word and a zeroed high
//! register keeps all eight bytes, so a caller that widens the printed result
//! gets the zero-extension the binary performs. Narrowed to the low word it
//! printed as `int`, and `(unsigned long long)ins16(0x7fff, 0)` became
//! `0xffffffff80000000` where the source returns `0x80000000`.
mod common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::process::Command;

/// The source of every image, built with `clang --target=<triple> -O2`.
const SOURCE: &str = r#"
static u64 s_ins16(unsigned a, unsigned short b) { return ((a + 1) << 16) | b; }
static u64 s_zadd(int a, int b) { return (unsigned)(a + b); }
static u64 s_zsh(int a) { return (unsigned)(a >> 3); }
static u64 s_zsel(int a, unsigned b) { return a > 0 ? b : 7u; }
static u64 s_zmix(unsigned a) { return a > 5 ? a : 1ULL << 40; }
static u64 s_twice(unsigned a, unsigned short b) { return s_ins16(a, b) * 2; }
static u64 s_plus1(int a, int b) { return s_zadd(a, b) + 1; }
"#;

const MAIN: &str = r#"
static const unsigned V[] = {0, 1, 5, 6, 7, 0x7fff, 0x8000, 0xffff, 0x10000, 0x7fffffff,
                             0x80000000, 0x80007fff, 0xfffffff8, 0xfffffffe, 0xffffffff,
                             0x12345678, 0xdeadbeef};
#define N (sizeof V / sizeof V[0])
#define CHECK(got, want) do { u64 g = (got), w = (want); if (g != w) { \
    printf("%s x=%#x y=%#x: %016llx, want %016llx\n", #got, x, y, g, w); bad++; } } while (0)
int main(void) {
  int bad = 0;
  for (unsigned i = 0; i < N; i++)
    for (unsigned j = 0; j < N; j++) {
      unsigned x = V[i], y = V[j];
      CHECK(ins16(x, (unsigned short)y), s_ins16(x, (unsigned short)y));
      CHECK(zadd(x, y), s_zadd(x, y));
      CHECK(zsh(x), s_zsh(x));
      CHECK(zsel(x, y), s_zsel(x, y));
      CHECK(zmix(x), s_zmix(x));
      CHECK(twice(x, (unsigned short)y), s_twice(x, (unsigned short)y));
      CHECK(plus1(x, y), s_plus1(x, y));
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

fn be_words(code: &[u32]) -> Vec<u8> {
    code.iter().flat_map(|w| w.to_be_bytes()).collect()
}

fn func(name: &'static str, code: &[u32]) -> Func {
    Func { name, code: words(code), call: None }
}

fn caller(name: &'static str, code: &[u32], word: usize, target: &'static str) -> Func {
    Func { name, code: words(code), call: Some((word * 4, target)) }
}

/// A little-endian relocatable object whose `.text` is `funcs` back to back,
/// each call encoded by `call(from, to)`.
fn object(arch: Architecture, e_flags: u32, funcs: &[Func], call: impl Fn(u64, u64) -> Vec<u8>) -> Vec<u8> {
    object_in(arch, Endianness::Little, e_flags, funcs, call)
}

fn object_in(
    arch: Architecture,
    endian: Endianness,
    e_flags: u32,
    funcs: &[Func],
    call: impl Fn(u64, u64) -> Vec<u8>,
) -> Vec<u8> {
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
    let mut object = Object::new(BinaryFormat::Elf, arch, endian);
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

/// `ins16` is `orr r0,r1,r0,lsl #16; mov r1,#0; add r0,r0,#65536; bx lr`.
fn arm_image() -> Vec<u8> {
    let funcs = [
        func("ins16", &[0xe1810800, 0xe3a01000, 0xe2800801, 0xe12fff1e]),
        func("zadd", &[0xe0810000, 0xe3a01000, 0xe12fff1e]),
        func("zsh", &[0xe1a001c0, 0xe3a01000, 0xe12fff1e]),
        func("zsel", &[0xe3500000, 0xd3a01007, 0xe1a00001, 0xe3a01000, 0xe12fff1e]),
        func(
            "zmix",
            &[0xe3500005, 0xe3a02000, 0xe3a01000, 0x81a02000, 0xe3500006, 0x33a01001, 0xe1a00002, 0xe1a01401, 0xe12fff1e],
        ),
        caller("twice", &[0xe92d4800, 0, 0xe1a01081, 0xe1811fa0, 0xe1a00080, 0xe8bd4800, 0xe12fff1e], 1, "ins16"),
        caller("plus1", &[0xe92d4800, 0, 0xe2900001, 0xe2a11000, 0xe8bd4800, 0xe12fff1e], 1, "zadd"),
    ];
    object(Architecture::Arm, 0x0500_0000, &funcs, |from, to| {
        (0xeb000000u32 | ((to as i64 - from as i64 - 8) >> 2) as u32 & 0xffffff).to_le_bytes().to_vec()
    })
}

/// `ins16` ends `jr $ra; addiu $3,$zero,0`: `$3`, the high word, is zeroed in
/// the delay slot. The loader lays `.text` out at 0x400000, which `jal` names.
fn mipsel_image() -> Vec<u8> {
    let funcs = [
        func("ins16", &[0x00040c00, 0x00250825, 0x3c020001, 0x00221021, 0x03e00008, 0x24030000]),
        func("zadd", &[0x00a41021, 0x03e00008, 0x24030000]),
        func("zsh", &[0x000410c3, 0x03e00008, 0x24030000]),
        func("zsel", &[0x28810001, 0x24020007, 0x00a1100a, 0x03e00008, 0x24030000]),
        func("zmix", &[0x2c810006, 0x0001200b, 0x00011a00, 0x03e00008, 0x00801025]),
        caller(
            "twice",
            &[
                0x27bdffe8, 0xafbf0014, 0, 0x00000000, 0x00020fc2, 0x00031840, 0x00611825, 0x00021040, 0x8fbf0014,
                0x03e00008, 0x27bd0018,
            ],
            2,
            "ins16",
        ),
        caller(
            "plus1",
            &[
                0x27bdffe8, 0xafbf0014, 0, 0x00000000, 0x24410001, 0x0022102b, 0x00621821, 0x00201025, 0x8fbf0014,
                0x03e00008, 0x27bd0018,
            ],
            2,
            "zadd",
        ),
    ];
    object(Architecture::Mips, 0x7000_1005, &funcs, |_, to| {
        (0x0c000000u32 | ((0x400000 + to) >> 2) as u32 & 0x3ffffff).to_le_bytes().to_vec()
    })
}

/// `ins16` is `rlwimi 4,3,16,0,15; addis 3,4,1; li 4,0; blr`.
fn ppcle_image() -> Vec<u8> {
    let funcs = [
        func("ins16", &[0x5064801e, 0x3c640001, 0x38800000, 0x4e800020]),
        func("zadd", &[0x7c641a14, 0x38800000, 0x4e800020]),
        func("zsh", &[0x7c631e70, 0x38800000, 0x4e800020]),
        func(
            "zsel",
            &[0x38a00007, 0x2c030000, 0x4181000c, 0x60a30000, 0x48000008, 0x38640000, 0x38800000, 0x4e800020],
        ),
        func(
            "zmix",
            &[
                0x38800000, 0x28030005, 0x28830006, 0x38a00100, 0x4181000c, 0x60830000, 0x48000004, 0x41840008,
                0x4e800020, 0x38850000, 0x4e800020,
            ],
        ),
        caller(
            "twice",
            &[
                0x7c0802a6, 0x90010004, 0x9421fff0, 0, 0x5465083e, 0x5085083c, 0x5463083c, 0x7ca42b78, 0x80010014,
                0x38210010, 0x7c0803a6, 0x4e800020,
            ],
            3,
            "ins16",
        ),
        caller(
            "plus1",
            &[
                0x7c0802a6, 0x90010004, 0x9421fff0, 0, 0x30630001, 0x7c840194, 0x80010014, 0x38210010, 0x7c0803a6,
                0x4e800020,
            ],
            3,
            "zadd",
        ),
    ];
    object(Architecture::PowerPc, 0, &funcs, |from, to| {
        (0x48000001u32 | (to as i64 - from as i64) as u32 & 0x03fffffc).to_le_bytes().to_vec()
    })
}

/// `ins16` ends `xor %edx,%edx; ret`.
fn i386_image() -> Vec<u8> {
    let hex = |s: &str| -> Vec<u8> {
        s.split_whitespace().map(|b| u8::from_str_radix(b, 16).unwrap()).collect()
    };
    let f = |name, code: &str, call: Option<(usize, &'static str)>| Func { name, code: hex(code), call };
    let funcs = [
        f("ins16", "0f b7 44 24 08 8b 4c 24 04 c1 e1 10 01 c8 05 00 00 01 00 31 d2 c3", None),
        f("zadd", "8b 44 24 08 03 44 24 04 31 d2 c3", None),
        f("zsh", "8b 44 24 04 c1 f8 03 31 d2 c3", None),
        f("zsel", "83 7c 24 04 00 b8 07 00 00 00 7e 04 8b 44 24 08 31 d2 c3", None),
        f("zmix", "8b 4c 24 04 31 c0 31 d2 83 f9 06 0f 43 c1 0f 92 c2 c1 e2 08 c3", None),
        f(
            "twice",
            "83 ec 0c 0f b7 44 24 14 83 ec 08 50 ff 74 24 1c e8 00 00 00 00 83 c4 10 0f a4 c2 01 01 c0 83 c4 0c c3",
            Some((16, "ins16")),
        ),
        f(
            "plus1",
            "83 ec 14 ff 74 24 1c ff 74 24 1c e8 00 00 00 00 83 c4 10 83 c0 01 83 d2 00 83 c4 0c c3",
            Some((11, "zadd")),
        ),
    ];
    object(Architecture::I386, 0, &funcs, |from, to| {
        let mut bytes = vec![0xe8];
        bytes.extend_from_slice(&((to as i64 - from as i64 - 5) as i32).to_le_bytes());
        bytes
    })
}

/// `ins16` ends `ret; restore %g0,%g0,%o0`: the window restore zeroes `%o0`,
/// the high word, and `%i1` comes back as `%o1`. Without the order in which
/// big-endian pairs are joined (#763) the printed values are not yet right, so
/// SPARC is checked for its return width only. `o0_1` names the whole pair, so
/// the value is a plain register, not a join.
fn sparc_image() -> Vec<u8> {
    let f = |name, code: &[u32]| Func { name, code: be_words(code), call: None };
    let funcs = [
        f("ins16", &[0x9de3bfa0, 0xb12e2010, 0xb0160019, 0x33000040, 0xb2060019, 0x81c7e008, 0x91e80000]),
        f("zadd", &[0x9de3bfa0, 0xb2064018, 0x81c7e008, 0x91e80000]),
        f("zsh", &[0x9de3bfa0, 0xb33e2003, 0x81c7e008, 0x91e80000]),
        f("zsel", &[0x9de3bfa0, 0x80a62000, 0x14800003, 0x01000000, 0xb2102007, 0x81c7e008, 0x91e80000]),
        f(
            "zmix",
            &[
                0x9de3bfa0, 0xb2100018, 0x80a62006, 0x0a800004, 0xb4100000, 0x10800003, 0xb010001a, 0xb0102001,
                0x80a66005, 0x18800003, 0xb12e2008, 0xb210001a, 0x81c7e008, 0x81e80000,
            ],
        ),
    ];
    object_in(Architecture::Sparc, Endianness::Big, 0, &funcs, |_, _| Vec::new())
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

const PRELUDE: &str = "#include <stdbool.h>\ntypedef unsigned long long u64;\n\
                       #define CONCAT44(h, l) ((u64)(unsigned int)(h) << 32 | (unsigned int)(l))\n";

/// Every function returns eight bytes on the 32-bit target itself, where `long`
/// is four: `text` with `sizeof` assertions, compiled with `-m32`.
const WIDTHS: &str = r#"
_Static_assert(sizeof ins16(0, 0) == 8, "ins16");
_Static_assert(sizeof zadd(0, 0) == 8, "zadd");
_Static_assert(sizeof zsh(0) == 8, "zsh");
_Static_assert(sizeof zsel(0, 0) == 8, "zsel");
_Static_assert(sizeof zmix(0) == 8, "zmix");
"#;

/// How an image's printed C names an eight-byte integer.
#[derive(Clone, Copy, PartialEq)]
enum Names {
    /// As C does on a 32-bit target (`long long`): checked under `-m32`.
    Ilp32,
    /// `long`: the MIPS and PowerPC specs declare no `long` size, so kuna's
    /// `long` is eight bytes there. Each function must be declared like `zmix`,
    /// whose value needs all 64 bits.
    Long8,
}

fn declaration<'a>(arch: &str, text: &'a str, name: &str) -> &'a str {
    text.lines()
        .find(|l| l.contains(&format!(" {name}(")) && !l.starts_with(' '))
        .unwrap_or_else(|| panic!("{arch}: no declaration of {name}:\n{text}"))
}

fn return_type<'a>(arch: &str, text: &'a str, name: &str) -> &'a str {
    declaration(arch, text, name).split(&format!(" {name}(")).next().unwrap()
}

#[test]
fn a_zero_extended_pair_return_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let images = [
        ("arm", arm_image(), Names::Ilp32, true),
        ("mipsel", mipsel_image(), Names::Long8, true),
        ("ppcle", ppcle_image(), Names::Long8, true),
        ("i386", i386_image(), Names::Ilp32, true),
        ("sparc", sparc_image(), Names::Ilp32, false),
    ];
    for (arch, bytes, names, run) in images {
        let text = decompile(&format!("zext-pair-{arch}"), &bytes);
        if names == Names::Long8 {
            let wide = return_type(arch, &text, "zmix");
            for name in ["ins16", "zadd", "zsh", "zsel"] {
                assert_eq!(return_type(arch, &text, name), wide, "{arch}: {name} is narrower than zmix:\n{text}");
            }
        } else {
            let src = common::scratch_file(&format!("zext-pair-{arch}-widths"), "c");
            std::fs::write(&src, format!("{PRELUDE}{text}\n{WIDTHS}")).unwrap();
            let out = Command::new(compilers[0])
                .args(["-std=gnu11", "-w", "-m32", "-fsyntax-only", src.to_str().unwrap()])
                .args(common::CC_GCC15_DEMOTE)
                .output()
                .expect("spawn the C compiler");
            assert!(
                out.status.success(),
                "{arch}: a function returns fewer than eight bytes on its 32-bit target:\n{}\n{text}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
        if !run {
            continue;
        }
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let src = common::scratch_file(&format!("zext-pair-{arch}-{cc}{level}"), "c");
                let exe = src.with_extension("exe");
                std::fs::write(&src, format!("#include <stdio.h>\n{PRELUDE}{text}\n{SOURCE}\n{MAIN}")).unwrap();
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
