//! A WebAssembly module is an input: `functions` and `decompile` read it, the
//! commands that need an object-file view refuse it by name, and a body the
//! lowering refuses is reported instead of mis-decompiled.

use crate::common::{self, process, run_kuna};

use std::process::Command;

use kuna_analysis::loader::format::wasm::synthetic::{i32_const, indexed, ModuleBuilder, I32};

fn fixture(name: &str) -> String {
    common::repo_root().join("tests/fixtures/wasm").join(name).to_str().unwrap().to_owned()
}

fn written(stem: &str, bytes: &[u8]) -> String {
    let path = common::scratch_file(stem, "wasm");
    std::fs::write(&path, bytes).unwrap();
    path.to_str().unwrap().to_owned()
}

fn decompile(module: &str, function: &str) -> String {
    let (stdout, stderr, code) = run_kuna(&["decompile", module, function]);
    assert_eq!(code, 0, "decompile {function}: {stderr}");
    stdout
}

#[test]
fn the_issue_module_lists_and_decompiles() {
    let module = fixture("mul_add.wasm");
    let (stdout, stderr, code) = run_kuna(&["functions", &module]);
    assert_eq!((code, stderr.as_str()), (0, ""));
    assert_eq!(stdout, "0x80000000\t__wasm_call_ctors\n0x80000010\tmul_add\n");
    assert_eq!(decompile(&module, "mul_add"), "int mul_add(int a0,int a1,int a2)\n{\n  return a1 * a0 + a2;\n}\n");
}

#[test]
fn imports_are_named_call_targets_and_not_bodies() {
    let module = fixture("calls.wasm");
    let (stdout, _, code) = run_kuna(&["functions", &module]);
    assert_eq!(code, 0);
    assert!(stdout.ends_with("0xf0000000\text_log\n0xf0000010\text_fill\n"), "{stdout}");
    let (all, stderr, code) = run_kuna(&["decompile-all", &module]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(all.matches("// Function: ").count(), 10, "{all}");
    assert!(!all.contains("// Function: ext_"), "{all}");
    assert!(all.contains("v1 = a1 - ext_log(a1);"), "{all}");
    assert!(all.contains("ext_fill((unsigned int)v1,a0);"), "{all}");
}

#[test]
fn read_and_disassemble_see_the_flat_image() {
    let module = fixture("calls.wasm");
    let (stdout, stderr, code) = run_kuna(&["read", &module, "0x10000", "--bytes", "8"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("03 00 00 00 01 00 00 00"), "{stdout}");
    let (stdout, stderr, code) = run_kuna(&["disassemble", &module, "mix", "--count", "3"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(
        stdout.ends_with("0x80000d70    20000000000000000100000000000000 local.get 0x1\n"),
        "{stdout}"
    );
}

#[test]
fn object_view_commands_refuse_a_module_by_name() {
    let module = fixture("mul_add.wasm");
    for command in [&["strings"][..], &["xrefs", "--to", "mul_add"], &["decompile-graph"]] {
        let mut args = vec![command[0], module.as_str()];
        args.extend_from_slice(&command[1..]);
        let (_, stderr, code) = run_kuna(&args);
        assert_ne!(code, 0, "{command:?}");
        assert!(stderr.contains("WebAssembly input has no object-file view"), "{command:?}: {stderr}");
    }
}

#[test]
fn a_module_decodes_as_no_other_language() {
    let (_, stderr, code) = run_kuna(&["decompile", &fixture("mul_add.wasm"), "mul_add", "--target", "x86:LE:64:default"]);
    assert_ne!(code, 0);
    assert!(stderr.contains("decodes only as Wasm:LE:32:default"), "{stderr}");
}

#[test]
fn a_refused_body_is_reported_and_the_rest_decompile() {
    let mut builder = ModuleBuilder::default();
    builder
        .function("vector", &[], &[], &[], &[0xfd, 0x0c])
        .function("plain", &[I32], &[I32], &[], &[indexed(0x20, 0), i32_const(3), vec![0x6c]].concat());
    let module = written("wasm-refused", &builder.build());
    let (all, stderr, code) = run_kuna(&["decompile-all", &module]);
    assert_eq!(code, 0, "{stderr}");
    assert!(
        stderr.contains("1 of 2 function bodies were not lowered")
            && stderr.contains("the first is function 0, which uses instruction 0xfd"),
        "{stderr}"
    );
    assert!(all.contains("void vector(void)\n{\n  unsupported();\n}"), "{all}");
    assert!(all.contains("return a0 * 3;"), "{all}");
}

#[test]
fn a_damaged_module_is_an_error_at_every_length() {
    let bytes = std::fs::read(fixture("mul_add.wasm")).unwrap();
    for len in [8, 9, 20, 0x90, 0x148, bytes.len() - 1] {
        let module = written("wasm-cut", &bytes[..len]);
        let (_, stderr, code) = run_kuna(&["functions", &module]);
        assert!(code == 0 || stderr.starts_with("error: "), "cut at {len}: {stderr}");
        assert!(!stderr.contains("panicked"), "cut at {len}: {stderr}");
    }
}

const HARNESS: &str = r#"
#include <stdio.h>
static unsigned long long rotl(unsigned long long a, unsigned long long n) { n &= 63; return a << n | a >> ((64 - n) & 63); }
int ext_log(int x) { return x * 7 + 1; }
void ext_fill(int *p, int v) { for (int i = 0; i < 4; i++) p[i] = v + i; }
#include "calls.c"
SIGNATURES
BODIES
static unsigned long long state = 88172645463325252ull;
static unsigned long long next(void) { state ^= state << 13; state ^= state >> 7; state ^= state << 17; return state; }
int main(void) {
  for (int round = 0; round < 2000; round++) {
    int a = (int)next(), b = (int)next(), n = (int)(next() % 13);
    unsigned long long x = next();
    int values[16];
    for (int i = 0; i < 16; i++) values[i] = (int)next();
    if (sum(values, n) != d_sum(values, n)) return puts("sum differs");
    if (pick(a % 7, b) != d_pick(a % 7, b)) return puts("pick differs");
    if (mix(x, a) != d_mix(x, a)) return puts("mix differs");
    if (favg(a, b) != d_favg(a, b)) return puts("favg differs");
  }
  return 0;
}
"#;

/// The printed C of four functions, compiled beside the source they were
/// built from, returns what the source returns on 2000 random inputs.
#[test]
fn the_printed_c_computes_what_the_source_computes() {
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    let module = fixture("calls.wasm");
    let mut signatures = String::new();
    let mut bodies = String::new();
    for name in ["sum", "pick", "mix", "favg"] {
        let printed = decompile(&module, name).replacen(&format!(" {name}("), &format!(" d_{name}("), 1);
        signatures.push_str(printed.lines().next().unwrap());
        signatures.push_str(";\n");
        bodies.push_str(&printed);
    }
    let source = HARNESS.replace("SIGNATURES", &signatures).replace("BODIES", &bodies);
    for cc in compilers {
        let c = common::scratch_file(&format!("wasm-roundtrip-{cc}"), "c");
        let exe = c.with_extension("exe");
        std::fs::write(&c, &source).unwrap();
        let include = common::repo_root().join("tests/fixtures/wasm");
        let built = Command::new(cc)
            .args(["-O1", "-w", "-I"])
            .arg(&include)
            .args(common::CC_GCC15_DEMOTE)
            .arg("-o")
            .arg(&exe)
            .arg(&c)
            .output()
            .unwrap();
        assert!(built.status.success(), "{cc} rejects the printed C:\n{}\n{source}", String::from_utf8_lossy(&built.stderr));
        let run = Command::new(&exe).output().unwrap();
        assert!(run.status.success(), "{cc}: {}\n{source}", String::from_utf8_lossy(&run.stdout));
    }
}
