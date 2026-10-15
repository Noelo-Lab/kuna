//! An address-taken stack record prints as a declared structure, and no partial
//! access prints Ghidra's `name._<off>_<size>_`, under the aggressive preset.
#![cfg(all(target_os = "linux", target_endian = "little"))]

use crate::common;
use common::process;
use std::process::Command;

/// `(base, memset)` of the four builds of `fixtures/stack_struct.c` in
/// `tests/stages/kuna-stackstruct.xml`: gcc -O0, gcc -O2, clang -O0, clang -O2.
const BUILDS: [(u64, Option<u64>); 4] =
    [(0x401000, None), (0x501000, None), (0x601000, Some(0x60117f)), (0x701000, None)];

fn image() -> std::path::PathBuf {
    let xml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/stages/kuna-stackstruct.xml"
    ));
    let chunk =
        regex::Regex::new(r#"<bytechunk[^>]*offset="(0x[0-9a-f]+)"[^>]*>([0-9a-f]+)</bytechunk>"#)
            .unwrap();
    let mut bytes = Vec::new();
    for captures in chunk.captures_iter(xml) {
        let offset = usize::from_str_radix(&captures[1][2..], 16).unwrap() - 0x401000;
        let hex = &captures[2];
        bytes.resize(offset, 0);
        bytes.extend(
            (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()),
        );
    }
    let path = common::scratch_file("stack-struct", "bin");
    std::fs::write(&path, bytes).unwrap();
    path
}

/// The helpers' entries in one build, read off the stage file's symbols.
fn helpers(base: u64) -> Vec<(String, u64)> {
    let xml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/stages/kuna-stackstruct.xml"
    ));
    let symbol = regex::Regex::new(r#"offset="0x([0-9a-f]+)" name="(fetch|setup|report)_"#).unwrap();
    symbol
        .captures_iter(xml)
        .map(|c| (c[2].to_owned(), u64::from_str_radix(&c[1], 16).unwrap()))
        .filter(|(_, a)| (a & !0xfffff) == (base & !0xfffff))
        .collect()
}

fn decompile(image: &std::path::Path, base: u64, memset: Option<u64>, mode: &str) -> String {
    let mut asserts = vec![format!("function 0x{base:x}=parse_header")];
    asserts.extend(helpers(base).into_iter().map(|(n, a)| format!("function 0x{a:x}={n}")));
    asserts.extend(memset.map(|a| format!("function 0x{a:x}=memset")));
    let entry = format!("0x{base:x}");
    let mut args = vec![
        "decompile",
        image.to_str().unwrap(),
        &entry,
        "--raw-image",
        "--base",
        "0x401000",
        "--target",
        "x86:LE:64:default:gcc",
        "--mode",
        mode,
        "--option",
        "structdefs",
        "on",
        "--assert-strict",
    ];
    for a in &asserts {
        args.extend(["--assert", a.as_str()]);
    }
    let (stdout, stderr, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{stderr}");
    stdout
}

fn compilers() -> Vec<&'static str> {
    let found: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!found.is_empty(), "a C compiler is required");
    found
}

const DECLARATIONS: &str = "#include <stddef.h>\n\
void *memset(void *p, int c, unsigned long n);\n\
void fetch(void *dst, unsigned long n);\n\
void setup(void *h);\n\
int report(unsigned int w, unsigned int h, unsigned long sum);\n";

#[test]
fn every_build_declares_the_record_and_prints_c() {
    let image = image();
    let partial = regex::Regex::new(r"\._[0-9]+_[0-9]+_").unwrap();
    let record = regex::Regex::new(r"(?m)^  struct_[0-9]+ v[0-9]+; //").unwrap();
    for (base, memset) in BUILDS {
        let code = decompile(&image, base, memset, "aggressive");
        assert!(!partial.is_match(&code), "0x{base:x}: {code}");
        assert!(record.is_match(&code), "0x{base:x}: {code}");
        assert!(code.contains("typedef struct struct_"), "0x{base:x}: {code}");
        assert!(code.contains(".field_0x4"), "0x{base:x}: {code}");
        let source = common::scratch_file("stack-struct", "c");
        std::fs::write(&source, format!("{DECLARATIONS}{code}")).unwrap();
        for cc in compilers() {
            let compiled = Command::new(cc)
                .args(["-std=gnu11", "-fsyntax-only", "-fno-strict-aliasing"])
                .args(common::CC_GCC15_DEMOTE)
                .arg(&source)
                .output()
                .unwrap();
            assert!(
                compiled.status.success(),
                "{cc} 0x{base:x}: {}\n{code}",
                String::from_utf8_lossy(&compiled.stderr)
            );
        }
    }
}

#[test]
fn reliable_keeps_ghidra_notation() {
    let code = decompile(&image(), 0x401000, None, "reliable");
    assert!(code.contains("._4_2_ != 3"), "{code}");
    assert!(!code.contains("struct_"), "{code}");
}

/// The clang -O0 build's record is the whole 24-byte header (memset's byte
/// count), so its emitted C can be run against the source's own helpers: a
/// member at the wrong offset or of the wrong sign returns another value.
#[test]
fn emitted_record_reads_the_fields_the_source_wrote() {
    let code = decompile(&image(), 0x601000, Some(0x60117f), "aggressive");
    assert!(code.contains("memset(&v1,0,0x18);"), "{code}");
    let program = format!(
        r#"
#include <stddef.h>
#include <string.h>
struct header {{ unsigned int magic; unsigned short version; unsigned char w, h; unsigned int count; unsigned char *pixels; }};
static const unsigned char bytes[8] = {{0x45, 0x48, 0x44, 0x52, 3, 0, 0xc8, 1}};
static unsigned char pixels[256];
void fetch(void *dst, unsigned long n) {{ for (unsigned long i = 0; i < n; i++) ((unsigned char *)dst)[i] = bytes[i]; }}
void setup(void *p) {{
    struct header *h = p;
    h->count = h->w * h->h;
    for (unsigned i = 0; i < h->count; i++) pixels[i] = (unsigned char)(i * 37);
    h->pixels = pixels;
}}
int report(unsigned int w, unsigned int h, unsigned long sum) {{ return (int)(w * 1000 + h * 100 + sum); }}
{code}
int main(void) {{
    unsigned long sum = 0;
    for (unsigned i = 0; i < 200; i++) sum += (unsigned char)(i * 37);
    return (int)parse_header() == (int)(200 * 1000 + 100 + sum) ? 0 : 1;
}}
"#
    );
    let source = common::scratch_file("stack-struct-run", "c");
    std::fs::write(&source, program).unwrap();
    for cc in compilers() {
        for optimization in ["-O0", "-O2"] {
            let executable = common::scratch_file("stack-struct-run", "exe");
            let compiled = Command::new(cc)
                .args(["-std=gnu11", optimization, "-fno-strict-aliasing"])
                .args(common::CC_GCC15_DEMOTE)
                .arg(&source)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                compiled.status.success(),
                "{cc} {optimization}: {}\n{code}",
                String::from_utf8_lossy(&compiled.stderr)
            );
            let run = Command::new(&executable).output().unwrap();
            assert!(run.status.success(), "{cc} {optimization}: {code}");
        }
    }
}

/// One 8-byte store zeroes two adjacent locals and one 8-byte load reloads
/// them, but each address reaches a call at its own start: two objects.
#[test]
fn two_escaped_addresses_stay_two_objects() {
    let fixture = common::fixture("stack_struct_adjacent_a64.o");
    let (code, stderr, status) = common::run_kuna(&[
        "decompile",
        &fixture,
        "adjacent",
        "--mode",
        "aggressive",
        "--option",
        "structdefs",
        "on",
    ]);
    assert_eq!(status, 0, "{stderr}");
    assert!(!code.contains("struct_"), "{code}");
    assert!(!regex::Regex::new(r"\._[0-9]+_[0-9]+_").unwrap().is_match(&code), "{code}");
}

/// A 16-byte piece reads through `unsigned __int128`, a plain 16-byte store or
/// copy is `memset`/`memcpy`, and nothing prints an array-typed value.
#[test]
fn wide_pieces_print_as_c() {
    let vm = common::fixture("arraycoverwidth_x86_64");
    let (code, stderr, status) = common::run_kuna(&["decompile", &vm, "vm"]);
    assert_eq!(status, 0, "{stderr}");
    for want in ["memset(&v2,0,0x10);", "memcpy(&v2,&v3[0],0x10);", "memcpy(&v1,&v2,0x10);"] {
        assert!(code.contains(want), "missing `{want}`:\n{code}");
    }
    let pair = common::fixture("zerofillfwd_a64.o");
    let (pair_code, stderr, status) = common::run_kuna(&["decompile-all", &pair]);
    assert_eq!(status, 0, "{stderr}");
    assert!(pair_code.contains("return *(unsigned __int128 *)v1;"), "{pair_code}");
    for text in [&code, &pair_code] {
        assert!(!text.contains("(*)["), "{text}");
        assert!(!regex::Regex::new(r"\._[0-9]+_[0-9]+_").unwrap().is_match(text), "{text}");
    }
}
