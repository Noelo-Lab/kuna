//! An `enum` stated with `--assert` is laid out and numbered as C lays it out:
//! `int`-wide unless a constant needs more, its unvalued constants counting on
//! from the one before, and as wide as a C23 underlying type when it names one.
mod common;
use common::process;

use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::path::PathBuf;
use std::process::Command;

/// `enum Mode { FIRST = 1, SECOND = 2 }; struct Packet { enum Mode mode; float
/// gain; int value; };` read back by clang -O2: `value` at +8, `gain` at +4.
const X86_64: [(&str, &[u8]); 3] = [
    ("read_value", &[0x8b, 0x47, 0x08, 0xc3]),
    ("read_gain", &[0xf3, 0x0f, 0x10, 0x47, 0x04, 0xc3]),
    ("read_mode", &[0x8b, 0x07, 0xc3]),
];
const AARCH64: [(&str, &[u8]); 3] = [
    ("read_value", &[0x00, 0x08, 0x40, 0xb9, 0xc0, 0x03, 0x5f, 0xd6]),
    ("read_gain", &[0x00, 0x04, 0x40, 0xbd, 0xc0, 0x03, 0x5f, 0xd6]),
    ("read_mode", &[0x00, 0x00, 0x40, 0xb9, 0xc0, 0x03, 0x5f, 0xd6]),
];
const I386: [(&str, &[u8]); 3] = [
    ("read_value", &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x08, 0xc3]),
    ("read_gain", &[0x8b, 0x44, 0x24, 0x04, 0xd9, 0x40, 0x04, 0xc3]),
    ("read_mode", &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x00, 0xc3]),
];

const PACKET: [&str; 5] = [
    "typedef enum Mode { FIRST = 1, SECOND = 2 };",
    "typedef struct Packet { enum Mode mode; float gain; int value; };",
    "prototype read_value int read_value(const struct Packet *packet)",
    "prototype read_gain float read_gain(const struct Packet *packet)",
    "prototype read_mode enum Mode read_mode(const struct Packet *packet)",
];

fn object_file(stem: &str, arch: Architecture, functions: &[(&str, &[u8])]) -> PathBuf {
    let mut obj = Object::new(BinaryFormat::Elf, arch, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    for (name, code) in functions {
        let value = obj.append_section_data(section, code, 16);
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size: code.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let path = common::scratch_file(stem, "o");
    std::fs::write(&path, obj.write().unwrap()).unwrap();
    path
}

/// Decompile `func` under `asserts` (all of which must apply) and return its C.
fn decompile(input: &PathBuf, func: &str, asserts: &[&str]) -> String {
    let mut args = vec!["decompile", input.to_str().unwrap(), func, "--json", "--assert-strict"];
    for a in asserts {
        args.extend(["--assert", a]);
    }
    let (stdout, stderr, code) = common::run_kuna(&args);
    assert_eq!(code, 0, "{func}: {stderr}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    for row in doc["assertions"].as_array().unwrap() {
        assert_eq!(row["status"], "applied", "{func}: {row}");
    }
    doc["functions"][0]["code"].as_str().unwrap().to_string()
}

/// The reporter's `Packet`: on a 64-bit target the enum was pointer-wide, so
/// `value` read as `gain` and `gain` as the second half of `mode`.  i386 is the
/// control, where the pointer happened to be as wide as `int`.
#[test]
fn an_asserted_enum_is_as_wide_as_int() {
    for (stem, arch, functions) in [
        ("enum-x86-64", Architecture::X86_64, &X86_64),
        ("enum-aarch64", Architecture::Aarch64, &AARCH64),
        ("enum-i386", Architecture::I386, &I386),
    ] {
        let input = object_file(stem, arch, functions);
        for (func, read) in [("read_value", "packet->value;"), ("read_gain", "packet->gain;"), ("read_mode", "packet->mode;")] {
            let code = decompile(&input, func, &PACKET);
            assert!(code.contains(&format!("return {read}")), "{stem} {func}: {code}");
        }
        std::fs::remove_file(input).unwrap();
    }
}

/// A constant that does not fit `int` makes the enum `long long`-wide on a
/// 32-bit target too, where the pointer-wide enum masked it away.
#[test]
fn a_constant_too_wide_for_int_widens_the_enum() {
    let input = object_file("enum-big-i386", Architecture::I386, &[("read_rec", I386[0].1)]);
    let code = decompile(
        &input,
        "read_rec",
        &[
            "typedef enum Big { SMALL = 1, HUGE = 0x100000000 };",
            "typedef struct Rec { enum Big kind; int value; };",
            "prototype read_rec int read_rec(const struct Rec *r)",
        ],
    );
    assert!(code.contains("return r->value;"), "{code}");
    std::fs::remove_file(input).unwrap();
}

/// `enum Color { RED, GREEN, BLUE }` is 0, 1, 2: the function returning 2
/// returns `BLUE`, where the constants were once numbered from 1.
#[test]
fn an_unvalued_enumerator_counts_on_from_the_one_before() {
    let input = object_file("enum-color", Architecture::X86_64, &[("blue", &[0xb8, 0x02, 0x00, 0x00, 0x00, 0xc3])]);
    let code = decompile(
        &input,
        "blue",
        &["typedef enum Color { RED, GREEN, BLUE };", "prototype blue enum Color blue(void)"],
    );
    assert!(code.contains("return BLUE;"), "{code}");
    std::fs::remove_file(input).unwrap();
}

/// `enum Neg2 { NA = -5, NB, NC, ND, NE, NF }; struct S10 { enum Neg2 n; char
/// c; int v; };` read back by clang -O2: `c` at +4, `v` at +8, `e == NB`.
const S10_X86_64: [(&str, &[u8]); 3] = [
    ("get_v", &[0x8b, 0x47, 0x08, 0xc3]),
    ("get_c", &[0x8a, 0x47, 0x04, 0xc3]),
    ("is_nb", &[0x31, 0xc0, 0x83, 0xff, 0xfc, 0x0f, 0x94, 0xc0, 0xc3]),
];
const S10_I386: [(&str, &[u8]); 3] = [
    ("get_v", &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x08, 0xc3]),
    ("get_c", &[0x8b, 0x44, 0x24, 0x04, 0x8a, 0x40, 0x04, 0xc3]),
    ("is_nb", &[0x31, 0xc0, 0x83, 0x7c, 0x24, 0x04, 0xfc, 0x0f, 0x94, 0xc0, 0xc3]),
];

const S10: [&str; 6] = [
    "typedef enum Neg2 { NA = -5, NB, NC, ND, NE, NF };",
    "typedef struct S10 { enum Neg2 n; char c; int v; };",
    "prototype get_v int get_v(struct S10 *p)",
    "prototype get_c char get_c(struct S10 *p)",
    "prototype is_nb int is_nb(enum Neg2 e)",
    "typedef enum Op { OP_NOP, OP_ADD, OP_SUB, OP_LAST = 2 };",
];

/// A negative constant makes the enum a signed `int`, as wide as `int` on
/// every target: it once made the enum eight bytes wide, on i386 too.  An
/// enumerator sharing another's constant is an alias C allows, not an error.
#[test]
fn a_negative_constant_keeps_the_enum_int_wide() {
    for (stem, arch, functions) in [
        ("enum-neg-x86-64", Architecture::X86_64, &S10_X86_64),
        ("enum-neg-i386", Architecture::I386, &S10_I386),
    ] {
        let input = object_file(stem, arch, functions);
        for (func, read) in [("get_v", "return p->v;"), ("get_c", "return p->c;"), ("is_nb", "e == NB")] {
            let code = decompile(&input, func, &S10);
            assert!(code.contains(read), "{stem} {func}: {code}");
        }
        std::fs::remove_file(input).unwrap();
    }
}

/// A `-fshort-enums` build states its one-byte enum with C23's underlying type.
#[test]
fn a_c23_underlying_type_sets_the_width() {
    let input = object_file(
        "enum-short",
        Architecture::X86_64,
        &[("read_h", &[0x0f, 0xbf, 0x47, 0x02, 0xc3]), ("read_s", &[0x8a, 0x07, 0xc3])],
    );
    let decls = [
        "typedef enum Small : unsigned char { S0, S1, S2 };",
        "typedef enum Flag : _Bool { OFF, ON };",
        "typedef struct Tiny { enum Small s; unsigned char b; short h; };",
        "prototype read_h int read_h(const struct Tiny *t)",
        "prototype read_s enum Small read_s(const struct Tiny *t)",
    ];
    assert!(decompile(&input, "read_h", &decls).contains("return t->h;"));
    assert!(decompile(&input, "read_s", &decls).contains("return t->s;"));
    std::fs::remove_file(input).unwrap();
}

/// The printed x86-64 functions, compiled against the same declarations,
/// read the members and compare the constants the machine code does.
#[test]
fn the_printed_readers_round_trip() {
    let input = object_file("enum-roundtrip", Architecture::X86_64, &X86_64);
    let mut printed: String = ["read_value", "read_gain", "read_mode"]
        .into_iter()
        .map(|f| decompile(&input, f, &PACKET) + "\n")
        .collect();
    std::fs::remove_file(input).unwrap();
    let input = object_file("enum-roundtrip-neg", Architecture::X86_64, &S10_X86_64);
    for f in ["get_v", "get_c", "is_nb"] {
        printed.push_str(&(decompile(&input, f, &S10) + "\n"));
    }
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let src = common::scratch_file("enum-roundtrip", "c");
    let exe = common::scratch_file("enum-roundtrip", "exe");
    std::fs::write(
        &src,
        format!(
            r#"
typedef enum Mode {{ FIRST = 1, SECOND = 2 }} Mode;
typedef struct Packet {{ Mode mode; float gain; int value; }} Packet;
typedef enum Neg2 {{ NA = -5, NB, NC, ND, NE, NF }} Neg2;
typedef struct S10 {{ Neg2 n; char c; int v; }} S10;
{printed}
int main(void) {{
    Packet p = {{ SECOND, 2.5f, 42 }};
    if (!(read_value(&p) == 42 && read_gain(&p) == 2.5f && read_mode(&p) == SECOND)) return 1;
    for (int x = -8; x < 8; x++) {{
        S10 s = {{ (Neg2)x, (char)(x * 3), x * 1000 }};
        if (get_v(&s) != x * 1000 || get_c(&s) != (char)(x * 3) || is_nb((Neg2)x) != (x == NB)) return 2;
    }}
    return 0;
}}
"#
        ),
    )
    .unwrap();
    for cc in &compilers {
        let compile = Command::new(cc).args(["-std=c11", "-O1"]).arg(&src).arg("-o").arg(&exe).output().unwrap();
        assert!(compile.status.success(), "{cc}: {}\n{printed}", String::from_utf8_lossy(&compile.stderr));
        assert!(Command::new(&exe).status().unwrap().success(), "{cc}: {printed}");
    }
    std::fs::remove_file(src).unwrap();
    std::fs::remove_file(exe).unwrap();
    std::fs::remove_file(input).unwrap();
}
