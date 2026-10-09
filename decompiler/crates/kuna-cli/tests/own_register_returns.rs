//! A function that keeps an argument in a callee-saved register across a call
//! and moves it back into its own argument register to return it (`mov r4,r1;
//! bl ext; mov r1,r4`) returns that register: the pair keeps both halves and the
//! argument stays a parameter. The ARM and Thumb builds are compiled back from
//! the printed C and compared with the source, as is a RISC-V 32 shift whose
//! zero-shift path moves only the high word back; the 128-bit AArch64 and
//! x86-64 returns print as a byte container, so those are checked by their text.
mod common;
use common::process;
use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, RelocationFlags, SectionKind, SymbolFlags,
    SymbolKind, SymbolScope,
};
use std::process::Command;

/// An -O2 build: `.text`, the functions in it, and where each call to the
/// undefined `ext` goes.
struct Image {
    arch: Architecture,
    e_flags: u32,
    text: &'static str,
    functions: &'static [(&'static str, u64, u64)],
    calls: &'static [u64],
    r_type: u32,
    addend: i64,
    thumb: bool,
}

const SOURCE32: &str = r#"
unsigned long long s_own(unsigned a, unsigned b) { ext(); return (unsigned long long)b << 32 | a; }
unsigned long long s_own_cond(unsigned a, unsigned b, int n) { if (n) ext(); return (unsigned long long)b << 32 | a; }
unsigned long long s_own_loop(unsigned a, unsigned b, int n) { for (int i = 0; i < n; i++) ext(); return (unsigned long long)b << 32 | a; }
unsigned long long s_swapd(unsigned a, unsigned b) { ext(); return (unsigned long long)a << 32 | b; }
int s_keep_int(int a, int b) { ext(); return a; }
unsigned long long s_two_ret(unsigned a, unsigned b, int n) { if (!n) return (unsigned long long)b << 32 | a; ext(); return (unsigned long long)b << 32 | a; }
unsigned long long s_two_ret_hi(unsigned a, unsigned b, int n) { if (!n) return (unsigned long long)b << 32 | 7; ext(); return (unsigned long long)b << 32 | a; }
"#;

/// The ARM image adds two hand-written functions that return the pair on two
/// paths, the early one leaving `r0` and `r1` untouched (`two_ret_hi` puts 7 in
/// `r0` first): `cmp r2,#0; [moveq r0,#7;] bxeq lr; push {r4,r5,r11,lr};
/// mov r4,r1; mov r5,r0; bl ext; mov r0,r5; mov r1,r4; pop {r4,r5,r11,lr}; bx lr`.
const ARM: Image = Image {
    arch: Architecture::Arm,
    e_flags: 0x0500_0000,
    text: "30482de90140a0e10050a0e1feffffeb0500a0e10410a0e13048bde81eff2fe1\
           30482de90140a0e10050a0e1000052e30000000afeffffeb0500a0e10410a0e1\
           3048bde81eff2fe170402de90140a0e10060a0e1010052e3030000ba0250a0e1\
           feffffeb015055e2fcffff1a0600a0e10410a0e17040bde81eff2fe130482de9\
           0140a0e10050a0e1feffffeb0400a0e10510a0e13048bde81eff2fe110402de9\
           0040a0e1feffffeb0400a0e11040bde81eff2fe1000052e31eff2f0130482de9\
           0140a0e10050a0e1feffffeb0500a0e10410a0e13048bde81eff2fe1000052e3\
           0700a0031eff2f0130482de90140a0e10050a0e1feffffeb0500a0e10410a0e1\
           3048bde81eff2fe1",
    functions: &[
        ("own", 0x00, 0x20),
        ("own_cond", 0x20, 0x28),
        ("own_loop", 0x48, 0x34),
        ("swapd", 0x7c, 0x20),
        ("keep_int", 0x9c, 0x18),
        ("two_ret", 0xb4, 0x28),
        ("two_ret_hi", 0xdc, 0x2c),
    ],
    calls: &[0x0c, 0x34, 0x60, 0x88, 0xa4, 0xc8, 0xf4],
    r_type: object::elf::R_ARM_CALL,
    addend: 0,
    thumb: false,
};

const THUMB: Image = Image {
    arch: Architecture::Arm,
    e_flags: 0x0500_0000,
    text: "b0b50c460546fff7feff28462146b0bdb0b5002a0c46054618bffff7feff2846\
           2146b0bd70b50c460646012a04db1546fff7feff013dfbd13046214670bdb0b5\
           0c460546fff7feff20462946b0bd10b50446fff7feff204610bd",
    functions: &[
        ("own", 0x00, 0x10),
        ("own_cond", 0x10, 0x14),
        ("own_loop", 0x24, 0x1a),
        ("swapd", 0x3e, 0x10),
        ("keep_int", 0x4e, 0x0c),
    ],
    calls: &[0x06, 0x1a, 0x30, 0x44, 0x52],
    r_type: object::elf::R_ARM_THM_PC22,
    addend: 0,
    thumb: true,
};

/// clang -O2 of `u128 own(unsigned long a, unsigned long b) { ext(); return
/// (u128)b << 64 | a; }`, `own3`, which returns `(u128)c << 64 | a`, and `long
/// keep_long(long a, long b, long c) { ext(); return a; }`.
const FUNCS64: &[(&str, u64, u64)] = &[("own", 0x00, 0x2c), ("own3", 0x2c, 0x2c), ("keep_long", 0x58, 0x24)];

const A64: Image = Image {
    arch: Architecture::Aarch64,
    e_flags: 0,
    text: "fd7bbea9f44f01a9fd030091f30301aaf40300aa00000094e00314aae10313aa\
           f44f41a9fd7bc2a8c0035fd6fd7bbea9f44f01a9fd030091f30302aaf40300aa\
           00000094e00314aae10313aaf44f41a9fd7bc2a8c0035fd6fd7bbea9f30b00f9\
           fd030091f30300aa00000094e00313aaf30b40f9fd7bc2a8c0035fd6",
    functions: FUNCS64,
    calls: &[0x14, 0x40, 0x68],
    r_type: object::elf::R_AARCH64_CALL26,
    addend: 0,
    thumb: false,
};

/// gcc -O2 -fcf-protection=none of the same source: `own` moves both arguments
/// into other registers, `own3` returns its third argument in `rdx`, where it
/// arrived.
const X64: Image = Image {
    arch: Architecture::X86_64,
    e_flags: 0,
    text: "41554989fd41544989f44883ec08e8000000004883c4084c89e84c89e2415c415d\
           c366662e0f1f8400000000000f1f0041554989fd41544989d44883ec08e8000000\
           004883c4084c89e84c89e2415c415dc366662e0f1f8400000000000f1f00415449\
           89fce8000000004c89e0415cc3",
    functions: &[("own", 0x00, 0x22), ("own3", 0x30, 0x22), ("keep_long", 0x60, 0x10)],
    calls: &[0x0f, 0x3f, 0x66],
    r_type: object::elf::R_X86_64_PLT32,
    addend: -4,
    thumb: false,
};

/// clang -O2 RISC-V 32 of compiler-rt's `__ashrdi3` shape ([`ASHR_SOURCE`]):
/// `mv a3,a1` copies the high word aside at entry, and the zero-shift path
/// returns `a` with `mv a1,a3; ret`, leaving `a0` untouched.
const RV32: Image = Image {
    arch: Architecture::Riscv32,
    e_flags: 0x5,
    text: "13770602ae8609ef1dc2b3d5c64013070002118fb396e6003355c500558d8280\
           93d5f641130506fe33d5a6408280b6858280",
    functions: &[("ashr", 0x00, 0x32)],
    calls: &[],
    r_type: 0,
    addend: 0,
    thumb: false,
};

const ASHR_SOURCE: &str = r#"
typedef long long di;
typedef union { di all; struct { unsigned low; int high; } s; } dw;
di s_ashr(di a, int b) {
  dw in, r; in.all = a;
  if (b & 32) { r.s.high = in.s.high >> 31; r.s.low = in.s.high >> (b - 32); }
  else { if (b == 0) return a; r.s.high = in.s.high >> b; r.s.low = (in.s.high << (32 - b)) | (in.s.low >> b); }
  return r.all;
}
"#;

fn object(image: &Image) -> Vec<u8> {
    let hex: String = image.text.split_whitespace().collect();
    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect();
    let mut obj = Object::new(BinaryFormat::Elf, image.arch, Endianness::Little);
    obj.flags = FileFlags::Elf {
        os_abi: 0,
        abi_version: 0,
        e_flags: image.e_flags,
    };
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &bytes, 16);
    for &(name, value, size) in image.functions {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value: value | u64::from(image.thumb),
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    let ext = obj.add_symbol(Symbol {
        name: b"ext".to_vec(),
        value: 0,
        size: 0,
        kind: SymbolKind::Unknown,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Undefined,
        flags: SymbolFlags::None,
    });
    for &offset in image.calls {
        obj.add_relocation(
            text,
            Relocation {
                offset,
                symbol: ext,
                addend: image.addend,
                flags: RelocationFlags::Elf {
                    r_type: image.r_type,
                },
            },
        )
        .unwrap();
    }
    obj.write().unwrap()
}

fn decompile(stem: &str, image: &Image) -> String {
    let path = common::scratch_file(stem, "o");
    std::fs::write(&path, object(image)).unwrap();
    let names: Vec<&str> = image.functions.iter().map(|f| f.0).collect();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile-all", path.to_str().unwrap(), "--mode", "aggressive", "--functions"])
        .arg(names.join(","))
        .output()
        .unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "{stem}: {text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::remove_file(path).unwrap();
    text
}

fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let marker = format!("// Function: {name} @");
    text.split(&marker)
        .nth(1)
        .unwrap_or_else(|| panic!("no {name}:\n{text}"))
        .split("// Function:")
        .next()
        .unwrap()
}

#[test]
fn arm_pairs_moved_back_into_their_own_registers_round_trip_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    for (stem, image, two) in [("own-return-arm", &ARM, true), ("own-return-thumb", &THUMB, false)] {
        let printed = decompile(stem, image);
        if two {
            for name in ["two_ret", "two_ret_hi"] {
                let decl = format!("unsigned long long {name}(unsigned int a0,unsigned int a1,int a2)");
                assert!(function(&printed, name).contains(&decl), "{stem}: `{decl}`\n{printed}");
            }
        }
        for (name, decl) in [
            ("own", "unsigned long long own(unsigned int a0,unsigned int a1)"),
            ("own_cond", "unsigned long long own_cond(unsigned int a0,unsigned int a1,int a2)"),
            ("own_loop", "unsigned long long own_loop(unsigned int a0,unsigned int a1,int a2)"),
            ("swapd", "unsigned long long swapd(unsigned int a0,unsigned int a1)"),
            ("keep_int", "unsigned int keep_int(unsigned int a0)"),
        ] {
            assert!(function(&printed, name).contains(decl), "{stem}: `{decl}`\n{printed}");
        }
        let src = common::scratch_file(stem, "c");
        let exe = common::scratch_file(stem, "exe");
        std::fs::write(
            &src,
            format!(
                r#"
#define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))
#define TWO {}
static int calls;
void ext(void) {{ calls++; }}
{printed}
{SOURCE32}
int main(void) {{
    unsigned v[][2] = {{{{5, 0x12345678u}}, {{0xffffffffu, 0xfedcba98u}}, {{0, 1}}, {{0x80000000u, 0}}}};
    for (unsigned i = 0; i < sizeof(v) / sizeof(*v); ++i) {{
        unsigned a = v[i][0], b = v[i][1];
        if (own(a, b) != s_own(a, b)) return 1;
        if (own_cond(a, b, 0) != s_own_cond(a, b, 0) || own_cond(a, b, 1) != s_own_cond(a, b, 1)) return 2;
        if (own_loop(a, b, 0) != s_own_loop(a, b, 0) || own_loop(a, b, 3) != s_own_loop(a, b, 3)) return 3;
        if (swapd(a, b) != s_swapd(a, b)) return 4;
        if ((int)keep_int(a) != s_keep_int((int)a, (int)b)) return 5;
#if TWO
        if (two_ret(a, b, 0) != s_two_ret(a, b, 0) || two_ret(a, b, 1) != s_two_ret(a, b, 1)) return 6;
        if (two_ret_hi(a, b, 0) != s_two_ret_hi(a, b, 0) || two_ret_hi(a, b, 1) != s_two_ret_hi(a, b, 1)) return 7;
#endif
    }}
    return 0;
}}
"#,
                u8::from(two)
            ),
        )
        .unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let compile = Command::new(cc)
                    .args(common::CC_GCC15_DEMOTE)
                    .args(["-std=gnu11", "-w", level, "-o"])
                    .arg(&exe)
                    .arg(&src)
                    .output()
                    .unwrap();
                assert!(
                    compile.status.success(),
                    "{stem} {cc} {level}: {}\n{printed}",
                    String::from_utf8_lossy(&compile.stderr)
                );
                let status = Command::new(&exe).status().unwrap();
                assert!(status.success(), "{stem} {cc} {level}: {status}\n{printed}");
            }
        }
        std::fs::remove_file(src).unwrap();
        std::fs::remove_file(exe).unwrap();
    }
}

#[test]
fn a_128_bit_pair_keeps_the_argument_returned_in_its_own_register() {
    for (stem, image, pairs) in [
        ("own-return-a64", &A64, &["own", "own3"][..]),
        ("own-return-x64", &X64, &["own", "own3"][..]),
    ] {
        let printed = decompile(stem, image);
        for &name in pairs {
            let body = function(&printed, name);
            let high = if name == "own" { "a1" } else { "a2" };
            let params = if name == "own" {
                "unsigned long a0,unsigned long a1"
            } else {
                "unsigned long a0,unsigned long a1,unsigned long a2"
            };
            assert!(body.contains(&format!("undefined16 {name}({params})")), "{stem}\n{printed}");
            assert!(body.contains(&format!("v1._8_8_ = {high};")), "{stem}\n{printed}");
            assert!(body.contains("v1._0_8_ = a0;"), "{stem}\n{printed}");
        }
        assert!(
            function(&printed, "keep_long").contains("unsigned long keep_long(unsigned long a0)"),
            "{stem}: a one-register return stays one register\n{printed}"
        );
    }
}

/// Moving back only the high word proves the pair is returned: the zero-shift
/// path hands back both words of `a`, never the high word alone.
#[test]
fn a_shift_that_returns_its_argument_unchanged_returns_both_words() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let stem = "own-return-rv32";
    let printed = decompile(stem, &RV32);
    let body = function(&printed, "ashr");
    assert!(
        body.contains("unsigned long long ashr(unsigned int a0,int a1,unsigned int a2)"),
        "{stem}\n{printed}"
    );
    assert!(body.contains("return CONCAT44(a1,a0);"), "{stem}\n{printed}");
    let src = common::scratch_file(stem, "c");
    let exe = common::scratch_file(stem, "exe");
    std::fs::write(
        &src,
        format!(
            r#"
#define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))
{printed}
{ASHR_SOURCE}
int main(void) {{
    unsigned long long v[] = {{0x1122334455667788ull, 0x8000000000000001ull, 0xffffffff00000000ull, 5}};
    for (unsigned i = 0; i < sizeof(v) / sizeof(*v); ++i)
        for (int b = 0; b < 64; ++b)
            if (ashr((unsigned)v[i], (int)(v[i] >> 32), b) != (unsigned long long)s_ashr((di)v[i], b)) return 1;
    return 0;
}}
"#
        ),
    )
    .unwrap();
    for cc in &compilers {
        for level in ["-O0", "-O2"] {
            let compile = Command::new(cc)
                .args(common::CC_GCC15_DEMOTE)
                .args(["-std=gnu11", "-w", level, "-o"])
                .arg(&exe)
                .arg(&src)
                .output()
                .unwrap();
            assert!(
                compile.status.success(),
                "{stem} {cc} {level}: {}\n{printed}",
                String::from_utf8_lossy(&compile.stderr)
            );
            let status = Command::new(&exe).status().unwrap();
            assert!(status.success(), "{stem} {cc} {level}: {status}\n{printed}");
        }
    }
    std::fs::remove_file(src).unwrap();
    std::fs::remove_file(exe).unwrap();
}
