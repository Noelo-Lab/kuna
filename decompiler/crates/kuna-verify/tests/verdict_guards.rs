//! What a verdict may claim. A run that traps on both sides is not an
//! agreement, a loop of empty blocks spends the budget, a store to a local
//! a callee is given is compared at the call, and the CLI never reports success
//! for work it did not do. The x86-64 fixtures:
//!
//! ```c
//! int quot(int a, int b) { return a / b; }   // mov eax,edi; cdq; idiv esi; ret
//! void spin(void) { for (;;) ; }              // jmp spin
//! void give(void) { int x = 1; sink(&x); }    // sub rsp,24; mov [rsp+12],1; lea rdi,[rsp+12]; call sink; ...
//! ```

use std::path::PathBuf;
use std::process::Command;

use kuna_verify::{check_binary, Options, Verdict};
use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, RelocationFlags, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};

fn specs() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs").canonicalize().unwrap().to_string_lossy().into_owned()
}

/// An object holding `.text` with the given functions, `(name, offset, size)`,
/// and a call to the undefined function named at each offset in `calls`.
fn object(name: &str, text: &[u8], functions: &[(&str, u64, u64)], calls: &[(u64, &str)]) -> PathBuf {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let code = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(code, text, 16);
    let sym = |obj: &mut Object, name: &str, value, size, section| {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section,
            flags: SymbolFlags::None,
        })
    };
    for &(f, at, size) in functions {
        sym(&mut obj, f, at, size, SymbolSection::Section(code));
    }
    let mut externs: Vec<(&str, object::write::SymbolId)> = Vec::new();
    for &(offset, callee) in calls {
        let symbol = match externs.iter().find(|(n, _)| *n == callee) {
            Some(&(_, id)) => id,
            None => {
                let id = sym(&mut obj, callee, 0, 0, SymbolSection::Undefined);
                externs.push((callee, id));
                id
            }
        };
        obj.add_relocation(code, Relocation { offset, symbol, addend: -4, flags: RelocationFlags::Elf { r_type: 4 } })
            .unwrap();
    }
    let dir = std::env::temp_dir().join(format!("kuna-verify-guards-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{name}.o"));
    crate::zext_pair::write_whole(&path, &obj.write().unwrap());
    path
}

fn quot() -> PathBuf {
    object("quot", &[0x89, 0xf8, 0x99, 0xf7, 0xfe, 0xc3], &[("quot", 0, 6)], &[])
}

fn check(path: &PathBuf, functions: &[&str], opts: &Options) -> Vec<kuna_verify::Report> {
    let functions: Vec<String> = functions.iter().map(|f| f.to_string()).collect();
    check_binary(&path.to_string_lossy(), &functions, &[], &[specs()], None, opts).unwrap()
}

#[test]
fn runs_that_trap_on_both_sides_are_not_counted_as_agreeing() {
    let reports = check(&quot(), &["quot"], &Options { runs: 64, ..Options::default() });
    let r = &reports[0];
    let Verdict::Agrees(n) = r.verdict else { panic!("{}", r.render()) };
    assert!(n > 0 && n < 64, "{}", r.render());
    assert!(r.lines.iter().any(|l| l.contains("trapped on both sides")), "{}", r.render());
}

#[test]
fn a_loop_of_empty_blocks_spends_the_budget() {
    let spin = object("spin", &[0xeb, 0xfe], &[("spin", 0, 2)], &[]);
    let reports = check(&spin, &["spin"], &Options { runs: 1, budget: 1000, ..Options::default() });
    assert_eq!(reports[0].verdict.label(), "UNKNOWN(budget)", "{}", reports[0].render());
}

/// The local a callee receives is compared at the call; the default build
/// stores it as the machine does.
#[test]
fn a_local_a_callee_receives_is_compared_at_the_call() {
    let text = [
        0x48, 0x83, 0xec, 0x18, 0xc7, 0x44, 0x24, 0x0c, 0x01, 0, 0, 0, 0x48, 0x8d, 0x7c, 0x24, 0x0c, 0xe8, 0, 0, 0, 0, 0x48,
        0x83, 0xc4, 0x18, 0xc3,
    ];
    let give = object("give", &text, &[("give", 0, text.len() as u64)], &[(18, "sink")]);
    let reports = check(&give, &["give"], &Options { runs: 16, ..Options::default() });
    assert_eq!(reports[0].verdict, Verdict::Agrees(16), "{}", reports[0].render());
}

/// A local whose address an earlier call kept is compared at later calls:
/// `save(&x); x = 1; consume();`.
#[test]
fn a_local_an_earlier_call_kept_is_compared_later() {
    let text = [
        0x48, 0x83, 0xec, 0x18, 0x48, 0x8d, 0x7c, 0x24, 0x0c, 0xe8, 0, 0, 0, 0, 0xc7, 0x44, 0x24, 0x0c, 0x01, 0, 0, 0, 0xe8,
        0, 0, 0, 0, 0x48, 0x83, 0xc4, 0x18, 0xc3,
    ];
    let kept = object("kept", &text, &[("kept", 0, text.len() as u64)], &[(10, "save"), (23, "consume")]);
    let reports = check(&kept, &["kept"], &Options { runs: 16, ..Options::default() });
    assert_eq!(reports[0].verdict, Verdict::Agrees(16), "{}", reports[0].render());
}

/// A function that fails to load gets its own verdict, and never a second
/// check of the function loaded before it.
#[test]
fn a_failed_load_does_not_check_the_previous_function_again() {
    let reports = check(&quot(), &["quot", "missing"], &Options { runs: 4, ..Options::default() });
    let names: Vec<&str> = reports.iter().map(|r| r.function.as_str()).collect();
    assert_eq!(names, ["quot", "missing"], "{reports:?}");
}

/// `dec` is `decompile`: an abbreviated command is checked too.
#[test]
fn an_abbreviated_decompile_is_checked() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../tests/datatests/pointersub.xml");
    let res = kuna_verify::check_datatest(&path.to_string_lossy(), &[specs()], &Options { runs: 4, ..Options::default() });
    assert!(res.reports.iter().any(|r| r.function == "getOuter"), "{:?}", res.error);
}

#[test]
fn a_missing_corpus_path_is_a_usage_error() {
    let out = verify(&["corpus", "no-such-directory", "--jobs", "1"]);
    assert_eq!(out.status.code(), Some(2), "{}", String::from_utf8_lossy(&out.stderr));
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

/// `check` over worker processes reports each function once, with the
/// verdict it gets in one process.
#[test]
fn chunked_workers_give_the_verdicts_of_one_process() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../kuna-analysis/tests/fixtures/regglobal_fmt_x86_64");
    let path = path.to_string_lossy().into_owned();
    let entries = kuna_verify::session::function_entries(&path, &[], &[specs()]).unwrap();
    let functions: Vec<String> = entries.iter().take(40).map(|e| format!("{e:#x}")).collect();
    let rows = |jobs: &str| {
        let mut args = vec!["check", path.as_str()];
        args.extend(functions.iter().map(String::as_str));
        args.extend(["--json", "--runs", "2", "--jobs", jobs, "--sleighpath"]);
        let specs = specs();
        args.push(&specs);
        let out = verify(&args);
        let mut v: Vec<String> = String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter(|l| l.starts_with("{\"function"))
            .map(|l| {
                let j: serde_json::Value = serde_json::from_str(l).unwrap();
                format!("{} {}", j["function"], j["label"])
            })
            .collect();
        v.sort();
        v
    };
    let one = rows("1");
    assert_eq!(one.len(), 40);
    assert_eq!(rows("3"), one);
}

/// Tier L's kuna side runs one instruction from the named registers: `sub rsp,
/// 8` moves the stack pointer down 8, `call rax` pushes the return address
/// and goes to `rax`, and the same seed gives the same lines.
#[test]
fn step_runs_one_instruction_from_the_named_registers() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../kuna-analysis/tests/fixtures/katavm_level1_x86_64");
    let path = path.to_string_lossy().into_owned();
    let specs = specs();
    let args = ["step", &path, "--regs", "RSP,RAX", "--flags", "CF,ZF", "--limit", "8", "--states", "1", "--canonical", "--sleighpath", &specs];
    let out = verify(&args);
    assert_eq!(out.status.code(), Some(0), "{}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let lines: Vec<serde_json::Value> = text.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
    let hex = |v: &serde_json::Value| u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap();
    let sub = lines.iter().find(|l| l["form"] == "sub RSP,#").expect("a sub rsp");
    assert_eq!(hex(&sub["out"]["RSP"]), hex(&sub["in"]["RSP"]) - 8);
    let call = lines.iter().find(|l| l["form"] == "call RAX").expect("a call rax");
    assert_eq!(hex(&call["next"]), hex(&call["in"]["RAX"]));
    assert_eq!(call["writes"].as_object().unwrap().len(), 8);
    assert_eq!(String::from_utf8_lossy(&verify(&args).stdout), text);
}
