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

/// C5: `x == 0x1badb002` is a branch no random run takes; with concolic
/// queries z3 gives the input that takes it, and one more run agrees. Skipped
/// where z3 is not installed.
#[test]
fn concolic_inputs_reach_a_branch_random_runs_miss() {
    if Command::new("z3").arg("-version").output().is_err() {
        return;
    }
    let text = [
        0x55, 0x48, 0x89, 0xe5, 0x89, 0xf9, 0x83, 0xe1, 0x01, 0x81, 0xff, 0x02, 0xb0, 0xad, 0x1b, 0xb8, 0x07, 0x00, 0x00, 0x00,
        0x0f, 0x45, 0xc1, 0x5d, 0xc3,
    ];
    let path = object("magic", &text, &[("magic", 0, text.len() as u64)], &[]);
    let report = |concolic| check(&path, &["magic"], &Options { runs: 64, concolic, ..Options::default() }).remove(0);
    assert_eq!(report(0).verdict.label(), "AGREES(64)");
    let r = report(8);
    assert_eq!(r.verdict.label(), "AGREES(65)");
    let s = r.concolic.unwrap();
    assert_eq!((s.models, s.solver), (1, kuna_verify::concolic::SolverState::Ran), "{}", r.render());
}

/// `--concolic` with no z3 to run says so, in the lines and the JSON, instead
/// of an `AGREES` that looks like the random runs and the concolic ones both
/// passed.
#[test]
fn concolic_without_z3_says_so() {
    let text = [
        0x55, 0x48, 0x89, 0xe5, 0x89, 0xf9, 0x83, 0xe1, 0x01, 0x81, 0xff, 0x02, 0xb0, 0xad, 0x1b, 0xb8, 0x07, 0x00, 0x00, 0x00,
        0x0f, 0x45, 0xc1, 0x5d, 0xc3,
    ];
    let path = object("magic_noz3", &text, &[("magic", 0, text.len() as u64)], &[]);
    let out = Command::new(env!("CARGO_BIN_EXE_kuna-verify"))
        .args(["check", &path.to_string_lossy(), "magic", "--runs", "16", "--concolic", "4", "--json", "--sleighpath", &specs()])
        .env("PATH", "")
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    let row: serde_json::Value = text.lines().find(|l| l.starts_with('{')).map(|l| serde_json::from_str(l).unwrap()).unwrap();
    assert_eq!(row["z3"]["solver"], "unavailable", "{text}");
    assert!(row["lines"].as_array().unwrap().iter().any(|l| l.as_str().unwrap().contains("z3 could not be started")), "{text}");
}

/// A call followed only by padding and then another function does not
/// return, whatever its callee: `die(1);` then `g`. The machine stops at it as
/// kuna's flow does, instead of running on into `g`.
#[test]
fn a_call_with_only_padding_after_it_does_not_return() {
    let text = [
        0xbf, 0x01, 0, 0, 0, 0xe8, 0, 0, 0, 0, 0x0f, 0x1f, 0x00, 0x66, 0x90, 0x90, 0xb8, 0x05, 0, 0, 0, 0xc3,
    ];
    let path = object("padded", &text, &[("f", 0, 16), ("g", 16, 6)], &[(6, "die")]);
    let reports = check(&path, &["f"], &Options { runs: 16, ..Options::default() });
    assert_eq!(reports[0].verdict, Verdict::Agrees(16), "{}", reports[0].render());
}

/// A corpus worker still running after `--file-seconds` is killed and its
/// file reported as an error, so one stuck file cannot hold a run up.
#[test]
fn a_corpus_worker_past_its_time_is_killed() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../tests/datatests");
    let files = [dir.join("pointersub.xml"), dir.join("ccmp.xml")].map(|p| p.to_string_lossy().into_owned());
    let specs = specs();
    let started = std::time::Instant::now();
    let out = verify(&[
        "corpus", &files[0], &files[1], "--jobs", "2", "--runs", "2000000", "--file-seconds", "1", "--sleighpath", &specs,
    ]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.status.code(), Some(3), "{text}");
    assert!(text.contains("timeout: no verdicts in 1 s"), "{text}");
    assert!(started.elapsed() < std::time::Duration::from_secs(60));
}

fn mutants(test: &str) -> Vec<serde_json::Value> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../tests").join(test).to_string_lossy().into_owned();
    let specs = specs();
    let out = verify(&["mutate", &path, "--jobs", "1", "--json", "--sleighpath", &specs]);
    String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok()).filter(|v| v.get("operator").is_some()).collect()
}

/// A direct call kuna names another function for is a difference, also where
/// the callee is outside the image (the datatest's bytes end before it).
#[test]
fn a_mutant_call_target_is_caught() {
    let rows = mutants("datatests/concat.xml");
    let targets: Vec<_> = rows.iter().filter(|r| r["operator"] == "call-target" && r["reached"] == true).collect();
    assert!(!targets.is_empty());
    assert!(targets.iter().all(|r| r["killed"] == true), "{targets:?}");
}

/// A call that passes fewer arguments than the declared prototype kuna gives
/// its callee is a difference (Noelo-Lab/kuna#1922's shape).
#[test]
fn a_call_short_of_its_declared_prototype_is_caught() {
    let rows = mutants("stages/kuna-narrowext.xml");
    let dropped: Vec<_> = rows
        .iter()
        .filter(|r| r["operator"] == "drop-arg" && r["reached"] == true && r["what"].as_str().unwrap_or("").contains("declared prototype kept"))
        .collect();
    assert!(!dropped.is_empty());
    assert!(dropped.iter().all(|r| r["killed"] == true), "{dropped:?}");
}

/// `report(int tag, const char *fmt, ...)` from `kuna-varargsave` (gcc -O2)
/// hands its `va_list` to `vsink`, declared and outside the image, so the save
/// area is compared at the call. kuna's `varargsave` folds the stores that fill
/// it; the decompiled side fills it as `va_start` does, so both passes agree.
#[test]
fn a_folded_register_save_area_is_filled_as_va_start_does() {
    let text = "4881ecd8000000488954243048894c24384c894424404c894c244884c074370f\
294424500f294c24600f295424700f299c24800000000f29a424900000000f29\
ac24a00000000f29b424b00000000f29bc24c0000000488d8424e0000000488d\
542408c7442408100000004889442410488d442420c744240c30000000488944\
2418e869ffffff4881c4d8000000c390";
    let pass = "<com>lo fu report</com><com>decompile</com>";
    let xml = format!(
        "<decompilertest><binaryimage arch=\"x86:LE:64:default:gcc\">\
<bytechunk space=\"ram\" offset=\"0x100010\">{text}</bytechunk>\
<symbol space=\"ram\" offset=\"0x100000\" name=\"vsink\"/>\
<symbol space=\"ram\" offset=\"0x100010\" name=\"report\"/></binaryimage><script>\
<com>parse line struct va_tag {{ uint4 gp_offset; uint4 fp_offset; void *overflow_arg_area; void *reg_save_area; }};</com>\
<com>parse line extern int4 vsink(int4 tag, char *fmt, va_tag *ap);</com>\
{pass}<com>option varargsave off</com>{pass}</script></decompilertest>"
    );
    let dir = std::env::temp_dir().join(format!("kuna-verify-guards-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("vararg-report.xml");
    std::fs::write(&path, xml).unwrap();
    let res = kuna_verify::check_datatest(&path.to_string_lossy(), &[specs()], &Options { runs: 32, ..Options::default() });
    assert_eq!(res.error, None);
    let labels: Vec<String> = res.reports.iter().map(|r| r.verdict.label()).collect();
    assert_eq!(labels, ["AGREES(32)", "AGREES(32)"], "{}", res.reports.iter().map(|r| r.render()).collect::<String>());
}

/// A call the machine makes through a register and kuna prints direct
/// (`lea rax,[rip+one]; call rax` as `one()`) has its target compared, and the
/// machine's callee effect is replayed only for the same callee: a mutant that
/// names `two` instead is caught.
#[test]
fn a_devirtualized_call_to_the_wrong_function_is_caught() {
    let text = [
        0xb8, 0x01, 0, 0, 0, 0xc3, 0xb8, 0x02, 0, 0, 0, 0xc3, 0x48, 0x83, 0xec, 0x08, 0x48, 0x8d, 0x05, 0xe9, 0xff, 0xff, 0xff,
        0xff, 0xd0, 0x83, 0xc0, 0x0a, 0x48, 0x83, 0xc4, 0x08, 0xc3,
    ];
    let path = object("devirt", &text, &[("one", 0, 6), ("two", 6, 6), ("callvia", 12, 21)], &[]);
    let specs = specs();
    let out = verify(&["mutate", &path.to_string_lossy(), "callvia", "--json", "--sleighpath", &specs]);
    let rows: Vec<serde_json::Value> =
        String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| serde_json::from_str(l).ok()).filter(|v: &serde_json::Value| v.get("operator").is_some()).collect();
    let targets: Vec<_> = rows.iter().filter(|r| r["operator"] == "call-target" && r["reached"] == true).collect();
    assert!(!targets.is_empty(), "{rows:?}");
    assert!(targets.iter().all(|r| r["killed"] == true), "{targets:?}");
}

/// A frame address stored at an unaligned address (a packed structure's
/// pointer field) still makes the object it points to reachable by a callee:
/// `x = 1; *(int **)0x600001 = &x; sink();` with `x = 0` planted is caught.
#[test]
fn a_pointer_stored_unaligned_still_escapes() {
    let text = [
        0x48, 0x83, 0xec, 0x28, 0xc7, 0x44, 0x24, 0x14, 0x01, 0, 0, 0, 0x48, 0x8d, 0x44, 0x24, 0x14, 0x3e, 0x48, 0x89, 0x04, 0x25,
        0x01, 0x00, 0x60, 0x00, 0xe8, 0, 0, 0, 0, 0x48, 0x83, 0xc4, 0x28, 0xc3,
    ];
    let path = object("unaligned", &text, &[("give", 0, text.len() as u64)], &[(27, "sink")]);
    let specs = specs();
    let out = verify(&["mutate", &path.to_string_lossy(), "give", "--per-function", "20", "--json", "--sleighpath", &specs]);
    let rows: Vec<serde_json::Value> =
        String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| serde_json::from_str(l).ok()).filter(|v: &serde_json::Value| v.get("operator").is_some()).collect();
    let planted: Vec<_> = rows.iter().filter(|r| r["what"].as_str().unwrap_or("").starts_with("constant 0x1 -> 0x0")).collect();
    assert_eq!(planted.len(), 1, "{rows:?}");
    assert_eq!(planted[0]["killed"], true, "{planted:?}");
}

/// A binary that cannot be loaded is a file that could not be checked (3), not
/// a usage error (2).
#[test]
fn a_binary_that_cannot_be_loaded_exits_3() {
    let missing = std::env::temp_dir().join(format!("kuna-verify-missing-{}.o", std::process::id()));
    let specs = specs();
    let out = verify(&["check", &missing.to_string_lossy(), "f", "--sleighpath", &specs]);
    assert_eq!(out.status.code(), Some(3), "{}", String::from_utf8_lossy(&out.stderr));
}

/// Two thunks that reach the same function are the same call only when they
/// leave the arguments alone: `add edi,1; jmp f` and `add edi,2; jmp f` are
/// not, so a mutant that calls the second instead of the first is caught.
#[test]
fn thunks_that_change_an_argument_are_different_calls() {
    let text = [
        0x89, 0xf8, 0xc3, 0x83, 0xc7, 0x01, 0xeb, 0xf8, 0x83, 0xc7, 0x02, 0xeb, 0xf3, 0x48, 0x83, 0xec, 0x08, 0xe8, 0xed, 0xff, 0xff,
        0xff, 0x48, 0x83, 0xc4, 0x08, 0xc3,
    ];
    let path = object("thunks", &text, &[("f", 0, 3), ("plus1", 3, 5), ("plus2", 8, 5), ("caller", 13, 14)], &[]);
    let specs = specs();
    let out = verify(&["mutate", &path.to_string_lossy(), "caller", "--json", "--sleighpath", &specs]);
    let rows: Vec<serde_json::Value> =
        String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| serde_json::from_str(l).ok()).filter(|v: &serde_json::Value| v.get("operator").is_some()).collect();
    let targets: Vec<_> = rows.iter().filter(|r| r["operator"] == "call-target" && r["reached"] == true).collect();
    assert!(!targets.is_empty(), "{rows:?}");
    assert!(targets.iter().all(|r| r["killed"] == true), "{targets:?}");
}

/// A callee that reads past the object kuna gives it (kuna types `[rsp]` as
/// four bytes, the callee reads `[rdi+4]`) is compared there too, when the
/// bytes are another of kuna's locals: `v2 = 4` planted for `v2 = 5` is caught.
#[test]
fn a_callee_reading_past_kuna_s_object_is_compared() {
    let text = [
        0x8b, 0x47, 0x04, 0xc3, 0x48, 0x83, 0xec, 0x18, 0xc6, 0x04, 0x24, 0x07, 0xc7, 0x44, 0x24, 0x04, 0x05, 0, 0, 0, 0x48, 0x89,
        0xe7, 0xe8, 0xe4, 0xff, 0xff, 0xff, 0x48, 0x83, 0xc4, 0x18, 0xc3,
    ];
    let path = object("pastobject", &text, &[("reader", 0, 4), ("give", 4, 29)], &[]);
    let specs = specs();
    let out = verify(&["mutate", &path.to_string_lossy(), "give", "--per-function", "20", "--json", "--sleighpath", &specs]);
    let rows: Vec<serde_json::Value> =
        String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| serde_json::from_str(l).ok()).filter(|v: &serde_json::Value| v.get("operator").is_some()).collect();
    let planted: Vec<_> = rows.iter().filter(|r| r["what"].as_str().unwrap_or("").starts_with("constant 0x5 -> 0x4")).collect();
    assert_eq!(planted.len(), 1, "{rows:?}");
    assert_eq!(planted[0]["killed"], true, "{planted:?}");
}

fn mutant_rows(path: &std::path::Path, function: &str) -> Vec<serde_json::Value> {
    let specs = specs();
    let out = verify(&["mutate", &path.to_string_lossy(), function, "--per-function", "20", "--json", "--sleighpath", &specs]);
    String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| serde_json::from_str(l).ok()).filter(|v: &serde_json::Value| v.get("operator").is_some()).collect()
}

/// Thunks that push a different argument before `jmp f` (`push 1; jmp f`,
/// `push 2; jmp f`, where `f` pops and returns it) are different calls: a
/// store or a stack-pointer change is never transparent.
#[test]
fn thunks_that_push_an_argument_are_different_calls() {
    let text = [
        0x59, 0x58, 0x51, 0xc3, 0x6a, 0x01, 0xeb, 0xf8, 0x6a, 0x02, 0xeb, 0xf4, 0x48, 0x83, 0xec, 0x08, 0xe8, 0xef, 0xff, 0xff,
        0xff, 0x48, 0x83, 0xc4, 0x08, 0xc3,
    ];
    let path = object("pushthunks", &text, &[("f", 0, 4), ("push1", 4, 4), ("push2", 8, 4), ("caller", 12, 14)], &[]);
    let rows = mutant_rows(&path, "caller");
    let targets: Vec<_> = rows.iter().filter(|r| r["operator"] == "call-target" && r["reached"] == true).collect();
    assert!(!targets.is_empty(), "{rows:?}");
    assert!(targets.iter().all(|r| r["killed"] == true), "{targets:?}");
}

/// A thunk that only jumps is transparent: calling `f` instead of `jt`
/// (`jt: jmp f`) is the same call, so that mutant is not reported.
#[test]
fn a_jump_only_thunk_is_the_function_it_jumps_to() {
    let text = [0xeb, 0x00, 0x89, 0xf8, 0xc3, 0x48, 0x83, 0xec, 0x08, 0xe8, 0xf2, 0xff, 0xff, 0xff, 0x48, 0x83, 0xc4, 0x08, 0xc3];
    let path = object("jumpthunk", &text, &[("jt", 0, 2), ("f", 2, 3), ("caller", 5, 14)], &[]);
    let rows = mutant_rows(&path, "caller");
    let targets: Vec<_> = rows.iter().filter(|r| r["operator"] == "call-target" && r["reached"] == true).collect();
    assert_eq!(targets.len(), 1, "{rows:?}");
    assert!(targets[0]["what"].as_str().unwrap_or("").contains("0x400000 -> 0x400002"), "{targets:?}");
    assert_eq!(targets[0]["killed"], false, "{targets:?}");
}

/// A loop bounded by its argument (`while (n) { r++; n--; }`) runs past the op
/// budget on both sides for a large `n`; those runs are counted apart, and the
/// runs that finish are compared.
#[test]
fn runs_past_the_budget_on_both_sides_are_counted_apart() {
    let text = [0x31, 0xc0, 0x85, 0xff, 0x74, 0x08, 0x83, 0xc0, 0x01, 0x83, 0xef, 0x01, 0xeb, 0xf4, 0xc3];
    let path = object("countdown", &text, &[("countdown", 0, text.len() as u64)], &[]);
    let reports = check(&path, &["countdown"], &Options { runs: 64, budget: 20_000, ..Options::default() });
    let r = &reports[0];
    let Verdict::Agrees(n) = r.verdict else { panic!("{}", r.render()) };
    assert!(n > 0 && n < 64, "{}", r.render());
    assert!(r.lines.iter().any(|l| l.contains("on both sides; they are not compared")), "{}", r.render());
}

/// Thunks that set a scratch register the callee then reads (`mov r10d,1;
/// jmp helper` against `mov r10d,2; jmp helper`, `helper: mov eax,r10d`) are
/// different calls: only the ABI's veneer registers may be written by a
/// transparent thunk.
#[test]
fn thunks_that_set_a_scratch_register_are_different_calls() {
    let text = [
        0x44, 0x89, 0xd0, 0xc3, 0x41, 0xba, 0x01, 0, 0, 0, 0xeb, 0xf4, 0x41, 0xba, 0x02, 0, 0, 0, 0xeb, 0xec, 0x48, 0x83, 0xec,
        0x08, 0xe8, 0xe7, 0xff, 0xff, 0xff, 0x48, 0x83, 0xc4, 0x08, 0xc3,
    ];
    let path = object("scratchthunks", &text, &[("helper", 0, 4), ("set1", 4, 8), ("set2", 12, 8), ("caller", 20, 14)], &[]);
    let rows = mutant_rows(&path, "caller");
    let targets: Vec<_> = rows.iter().filter(|r| r["operator"] == "call-target" && r["reached"] == true).collect();
    assert!(!targets.is_empty(), "{rows:?}");
    assert!(targets.iter().all(|r| r["killed"] == true), "{targets:?}");
}
