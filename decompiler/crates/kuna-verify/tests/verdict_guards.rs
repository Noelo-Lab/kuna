//! What a verdict may claim. A run that traps on both sides is not an
//! agreement, and the CLI never reports success for work it did not do.
//!
//! ```c
//! int quot(int a, int b) { return a / b; }   // x86-64: mov eax,edi; cdq; idiv esi; ret
//! ```

use std::path::PathBuf;
use std::process::Command;

use kuna_verify::{check_binary, Options, Verdict};
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};

fn specs() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs").canonicalize().unwrap().to_string_lossy().into_owned()
}

fn quot() -> PathBuf {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let code = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(code, &[0x89, 0xf8, 0x99, 0xf7, 0xfe, 0xc3], 16);
    obj.add_symbol(Symbol {
        name: b"quot".to_vec(),
        value: 0,
        size: 6,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(code),
        flags: SymbolFlags::None,
    });
    let dir = std::env::temp_dir().join(format!("kuna-verify-guards-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("quot.o");
    std::fs::write(&path, obj.write().unwrap()).unwrap();
    path
}

#[test]
fn runs_that_trap_on_both_sides_are_not_counted_as_agreeing() {
    let opts = Options { runs: 64, ..Options::default() };
    let reports = check_binary(&quot().to_string_lossy(), &["quot".to_string()], &[], &[specs()], None, &opts).unwrap();
    let r = &reports[0];
    let Verdict::Agrees(n) = r.verdict else { panic!("{}", r.render()) };
    assert!(n > 0 && n < 64, "{}", r.render());
    assert!(r.lines.iter().any(|l| l.contains("trapped on both sides")), "{}", r.render());
}

fn verify(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_kuna-verify")).args(args).output().unwrap()
}

#[test]
fn zero_runs_is_a_usage_error() {
    let out = verify(&["check", "nothing.o", "f", "--runs", "0"]);
    assert_eq!(out.status.code(), Some(2), "{}", String::from_utf8_lossy(&out.stderr));
}

#[test]
fn a_file_that_cannot_be_checked_fails_the_corpus() {
    let dir = std::env::temp_dir().join(format!("kuna-verify-badxml-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let bad = dir.join("broken.xml");
    std::fs::write(&bad, "<decompilertest><binaryimage arch=\"nosuch:LE:64:default\"></binaryimage><script><com>quit</com></script></decompilertest>").unwrap();
    let out = verify(&["corpus", &bad.to_string_lossy(), "--sleighpath", &specs(), "--jobs", "1"]);
    assert_eq!(out.status.code(), Some(3), "{}", String::from_utf8_lossy(&out.stdout));
}
