//! Every label in the printed C is followed by a statement, so the output
//! compiles as C11 with `-pedantic-errors` and behaves like the source: an
//! empty last switch arm (ARM and x86-64) and a goto target at the end of a
//! loop body (x86-64). The empty arm's Rust `match` form compiles too.
use crate::common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::process::Command;

struct Case {
    name: &'static str,
    architecture: Architecture,
    stage: &'static str,
    symbols: &'static [(&'static str, u64, u64)],
    prototypes: &'static [(&'static str, &'static str)],
    rust_functions: &'static [&'static str],
    c_main: &'static str,
    rust_main: &'static str,
}

const CASES: &[Case] = &[
    Case {
        name: "arm",
        architecture: Architecture::Arm,
        stage: include_str!("../../../../tests/stages/kuna-labelstmt-arm.xml"),
        symbols: &[("probe", 0, 52), ("probe_control", 0x40, 56)],
        prototypes: &[
            ("probe", "void probe(unsigned int selector, unsigned int *output)"),
            (
                "probe_control",
                "void probe_control(unsigned int selector, unsigned int *output)",
            ),
        ],
        rust_functions: &["probe", "probe_control"],
        c_main: r#"
int main(void) {
    static const unsigned want[] = {7, 9, 0, 0, 0}, control[] = {7, 9, 11, 0, 0};
    for (unsigned s = 0; s < 5; ++s) {
        unsigned out = 99, out2 = 99;
        probe(s, &out);
        probe_control(s, &out2);
        if (out != want[s] || out2 != control[s]) return 1 + (int)s;
    }
    unsigned out = 99;
    probe(0xffffffffu, &out);
    return out != 0;
}
"#,
        rust_main: r#"
fn main() { unsafe {
    for (s, want, control) in [(0u32, 7u32, 7u32), (1, 9, 9), (2, 0, 11), (3, 0, 0), (u32::MAX, 0, 0)] {
        let (mut out, mut out2) = (99u32, 99u32);
        probe(s, &mut out);
        probe_control(s, &mut out2);
        assert_eq!((out, out2), (want, control));
    }
} }
"#,
    },
    Case {
        name: "x64",
        architecture: Architecture::X86_64,
        stage: include_str!("../../../../tests/stages/kuna-labelstmt-x64.xml"),
        symbols: &[("last_case", 0, 0x82), ("skip_blanks", 0x90, 0x26)],
        prototypes: &[
            ("last_case", "void last_case(unsigned int selector, unsigned int *out)"),
            ("skip_blanks", "void skip_blanks(char **cpp)"),
        ],
        rust_functions: &["last_case"],
        c_main: r#"
static unsigned expect(unsigned s) {
    switch (s) {
    case 1: return 7;
    case 3: return 9;
    case 4: return 13;
    case 6: return 5;
    case 7: return 1;
    default: return 21;
    }
}
int main(void) {
    for (unsigned s = 0; s < 10; ++s) {
        unsigned out = 99;
        last_case(s, &out);
        if (out != expect(s)) return 1 + (int)s;
    }
    unsigned out = 99;
    last_case(0xffffffffu, &out);
    if (out != 21) return 20;
    char blanks[] = " \t  \tx y", plain[] = "x", empty[] = "";
    char *p = blanks;
    skip_blanks(&p);
    if (p != blanks + 5) return 21;
    p = plain;
    skip_blanks(&p);
    if (p != plain) return 22;
    p = empty;
    skip_blanks(&p);
    return p != empty;
}
"#,
        rust_main: r#"
fn main() { unsafe {
    for s in 0u32..10 {
        let want = match s { 1 => 7, 3 => 9, 4 => 13, 6 => 5, 7 => 1, _ => 21 };
        let mut out = 99u32;
        last_case(s, &mut out);
        assert_eq!(out, want);
    }
} }
"#,
    },
];

fn stage_bytes(stage: &str) -> Vec<u8> {
    let hex = stage
        .split("offset=\"0x1000\">")
        .nth(1)
        .unwrap()
        .split("</bytechunk>")
        .next()
        .unwrap();
    hex.as_bytes()
        .chunks_exact(2)
        .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
        .collect()
}

fn object_file(case: &Case) -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, case.architecture, Endianness::Little);
    if case.architecture == Architecture::Arm {
        obj.flags = FileFlags::Elf {
            os_abi: 0,
            abi_version: 0,
            e_flags: 0x0500_0000,
        };
    }
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &stage_bytes(case.stage), 16);
    for &(symbol, value, size) in case.symbols {
        obj.add_symbol(Symbol {
            name: symbol.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    obj.write().unwrap()
}

fn decompile(case: &Case, input: &std::path::Path, functions: &[&str], language: &str) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.args(["decompile-all", input.to_str().unwrap(), "--functions"])
        .arg(functions.join(","))
        .args(["--language", language]);
    for (function, prototype) in case.prototypes {
        if functions.contains(function) {
            cmd.args(["--assert", &format!("prototype {function} {prototype}")]);
        }
    }
    let output = cmd.arg("--assert-strict").output().unwrap();
    assert!(
        output.status.success(),
        "{}: {}",
        case.name,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// The first label line (`case N:`, `default:`, `name:`) that only blank lines
/// separate from a closing brace.
fn dangling_label(printed: &str) -> Option<&str> {
    let lines: Vec<&str> = printed.lines().map(str::trim).collect();
    lines.iter().enumerate().find_map(|(i, line)| {
        let is_label = line.ends_with(':') && !line.starts_with("//");
        let next = lines[i + 1..].iter().find(|l| !l.is_empty());
        (is_label && next.is_some_and(|l| l.starts_with('}'))).then_some(*line)
    })
}

#[test]
fn labels_are_followed_by_statements_and_compile_as_c11() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    for case in CASES {
        let input = common::scratch_file("label-statements", "o");
        let src = common::scratch_file("label-statements", "c");
        let exe = common::scratch_file("label-statements", "exe");
        std::fs::write(&input, object_file(case)).unwrap();
        let functions: Vec<_> = case.symbols.iter().map(|s| s.0).collect();
        let printed = decompile(case, &input, &functions, "c");
        assert_eq!(dangling_label(&printed), None, "{}: {printed}", case.name);
        std::fs::write(&src, format!("{printed}\n{}", case.c_main)).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let compile = Command::new(cc)
                    .args(["-std=c11", "-pedantic-errors", "-Werror", level])
                    .arg(&src)
                    .arg("-o")
                    .arg(&exe)
                    .output()
                    .unwrap();
                assert!(
                    compile.status.success(),
                    "{} {cc} {level}: {}\n{printed}",
                    case.name,
                    String::from_utf8_lossy(&compile.stderr)
                );
                let status = Command::new(&exe).status().unwrap();
                assert!(status.success(), "{} {cc} {level}: {status}\n{printed}", case.name);
            }
        }
        let rust = decompile(case, &input, case.rust_functions, "rust");
        let rust_src = common::scratch_file("label-statements", "rs");
        std::fs::write(&rust_src, format!("{rust}\n{}", case.rust_main)).unwrap();
        let compile = Command::new("rustc")
            .args(["--crate-name", "label_statements", "--edition", "2021"])
            .arg(&rust_src)
            .arg("-o")
            .arg(&exe)
            .output()
            .unwrap();
        assert!(
            compile.status.success(),
            "{} rustc: {}\n{rust}",
            case.name,
            String::from_utf8_lossy(&compile.stderr)
        );
        assert!(Command::new(&exe).status().unwrap().success(), "{} rustc: {rust}", case.name);
        for file in [input, src, exe, rust_src] {
            std::fs::remove_file(file).unwrap();
        }
    }
}
