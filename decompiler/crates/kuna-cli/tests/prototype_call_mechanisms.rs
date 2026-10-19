//! `callmechanisms`: compiler-declared prototype boundary effects.
use crate::common;
use common::process;
use std::process::Command;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn declared_return_effects_preserve_the_x87_stack_across_consecutive_calls() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|compiler| {
            process::optional_output(Command::new(compiler).arg("--version")).is_some()
        })
        .collect();
    assert!(!compilers.is_empty(), "a C compiler is required");
    let root = common::scratch_file("prototype-call-effects", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let specs = root.join("specs");
    std::fs::create_dir(&specs).unwrap();
    let stock = common::repo_root().join("specs/Ghidra/Processors/x86/data/languages");
    for entry in std::fs::read_dir(&stock).unwrap() {
        let path = entry.unwrap().path();
        if matches!(
            path.extension().and_then(|value| value.to_str()),
            Some("ldefs" | "pspec" | "cspec" | "sla")
        ) {
            std::fs::copy(&path, specs.join(path.file_name().unwrap())).unwrap();
        }
    }
    let base = std::fs::read_to_string(stock.join("x86-64-gcc.cspec")).unwrap();
    let mechanism = "<pcode inject=\"uponreturn\"><body>ST1=ST2; ST2=ST3; ST3=ST4; ST4=ST5; ST5=ST6; ST6=ST7;</body></pcode>";
    let model = r#"<prototype name="pair_stack" extrapop="8" stackshift="8">
<input><pentry minsize="10" maxsize="16" metatype="float"><register name="ST0"/></pentry><pentry minsize="10" maxsize="16" metatype="float"><register name="ST1"/></pentry></input>
<output><pentry minsize="10" maxsize="16" metatype="float"><register name="ST0"/></pentry></output>
<unaffected><register name="RSP"/><register name="RDI"/><register name="RSI"/><register name="ST2"/><register name="ST3"/><register name="ST4"/><register name="ST5"/><register name="ST6"/><register name="ST7"/></unaffected>
MECHANISM</prototype>"#;
    let assembly = root.join("sample.s");
    std::fs::write(
        &assembly,
        r#".text
.globl sample
.type sample,@function
sample:
 fldl 24(%rdi)
 fldl 16(%rdi)
 fldl 0(%rdi)
 fldl 8(%rdi)
 call pair
 call pair
 faddp %st,%st(1)
 fstpl (%rsi)
 ret
.size sample,.-sample
.globl pair
.protected pair
.type pair,@function
pair:
 faddp %st,%st(1)
 ret
.size pair,.-pair
.section .note.GNU-stack,"",@progbits
"#,
    )
    .unwrap();
    let original = root.join("original.so");
    let built = Command::new(compilers[0])
        .args(["-shared", "-fPIC"])
        .arg(&assembly)
        .arg("-o")
        .arg(&original)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let driver = root.join("driver.c");
    std::fs::write(&driver, include_str!("prototype_call_mechanisms_driver.c")).unwrap();
    let client = root.join("driver");
    let built = Command::new(compilers[0])
        .args(["-std=gnu11", "-O2"])
        .arg(&driver)
        .args(["-ldl", "-lm", "-o"])
        .arg(&client)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    for enabled in [false, true] {
        let custom = model.replace("MECHANISM", if enabled { mechanism } else { "" });
        std::fs::write(
            specs.join("x86-64-gcc.cspec"),
            base.replace("</compiler_spec>", &(custom + "</compiler_spec>")),
        )
        .unwrap();
        let (output, error, status) = common::run_kuna(&[
            "decompile-all",
            original.to_str().unwrap(),
            "--functions",
            "sample",
            "--sleighpath",
            specs.to_str().unwrap(),
            "--json",
            "--option",
            "nanignore",
            "none",
            "--option",
            "callmechanisms",
            "on",
            "--assert",
            "prototype sample void sample(double *values,double *output)",
            "--assert",
            "prototype pair long double pair_stack pair(long double left,long double right)",
        ]);
        assert_eq!(status, 0, "{error}");
        let document: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(document["functions"][0]["error"].is_null(), "{output}");
        let code = document["functions"][0]["code"].as_str().unwrap();
        let regenerated = root.join("regenerated.c");
        std::fs::write(&regenerated, format!("#define pair_stack\nlong double pair(long double left,long double right) {{ return left+right; }}\n{code}\n")).unwrap();
        for compiler in &compilers {
            for level in ["-O0", "-O2"] {
                let binary = root.join(format!("regenerated-{enabled}-{compiler}-{level}.so"));
                let built = Command::new(compiler)
                    .args([
                        level,
                        "-shared",
                        "-fPIC",
                        "-Werror=implicit-function-declaration",
                    ])
                    .arg(&regenerated)
                    .arg("-o")
                    .arg(&binary)
                    .output()
                    .unwrap();
                assert!(
                    built.status.success(),
                    "{code}\n{}",
                    String::from_utf8_lossy(&built.stderr)
                );
                let run = Command::new(&client)
                    .arg(&original)
                    .arg(&binary)
                    .output()
                    .unwrap();
                assert_eq!(
                    run.status.code(),
                    Some(if enabled { 0 } else { 1 }),
                    "enabled={enabled}, {compiler} {level}\n{code}\n{}",
                    String::from_utf8_lossy(&run.stderr)
                );
            }
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn return_payload_stores_and_unique_pointer_temporaries_survive_ssa() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|compiler| {
            process::optional_output(Command::new(compiler).arg("--version")).is_some()
        })
        .collect();
    assert!(!compilers.is_empty(), "a C compiler is required");
    let root = common::scratch_file("prototype-memory-effects", "dir");
    let specs = root.join("specs");
    std::fs::create_dir_all(&specs).unwrap();
    let stock = common::repo_root().join("specs/Ghidra/Processors/x86/data/languages");
    for entry in std::fs::read_dir(&stock).unwrap() {
        let path = entry.unwrap().path();
        if matches!(
            path.extension().and_then(|value| value.to_str()),
            Some("ldefs" | "pspec" | "cspec" | "sla")
        ) {
            std::fs::copy(&path, specs.join(path.file_name().unwrap())).unwrap();
        }
    }
    let base = std::fs::read_to_string(stock.join("x86-64-gcc.cspec")).unwrap();
    let model = r#"<prototype name="poke_effect" extrapop="8" stackshift="8">
<input><pentry minsize="1" maxsize="8"><register name="RDI"/></pentry></input><output/>
<unaffected><register name="RSP"/><register name="RDI"/><register name="RBX"/><register name="RBP"/><register name="R12"/><register name="R13"/><register name="R14"/><register name="R15"/></unaffected>
MECHANISM</prototype>"#;
    let mechanism = "<pcode inject=\"uponreturn\"><body>local pointer:8 = RDI + 4; *:4 pointer = 9;</body></pcode>";
    let assembly = root.join("sample.s");
    std::fs::write(
        &assembly,
        r#".text
.globl sample
.type sample,@function
sample:
 push %rbx
 mov %rdi,%rbx
 call poke
 mov 4(%rbx),%eax
 pop %rbx
 ret
.size sample,.-sample
.globl poke
.protected poke
.type poke,@function
poke:
 movl $9,4(%rdi)
 ret
.size poke,.-poke
.section .note.GNU-stack,"",@progbits
"#,
    )
    .unwrap();
    let original = root.join("original.so");
    let built = Command::new(compilers[0])
        .args(["-shared", "-fPIC"])
        .arg(&assembly)
        .arg("-o")
        .arg(&original)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let driver = root.join("driver.c");
    std::fs::write(
        &driver,
        r#"
#include <dlfcn.h>
#include <string.h>
typedef unsigned (*sample_fn)(unsigned *);
int main(int argc, char **argv) {
    if (argc != 3) return 2;
    void *first=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL), *second=dlopen(argv[2],RTLD_NOW|RTLD_LOCAL);
    if (!first || !second) return 2;
    sample_fn reference=(sample_fn)dlsym(first,"sample"), actual=(sample_fn)dlsym(second,"sample");
    if (!reference || !actual) return 2;
    for (unsigned i=0; i<256; ++i) {
        unsigned expected[3]={i,100+i,~i}, observed[3];
        memcpy(observed,expected,sizeof expected);
        unsigned a=reference(expected), b=actual(observed);
        if (a!=b || memcmp(expected,observed,sizeof expected)) return 1;
    }
    dlclose(first); dlclose(second); return 0;
}
"#,
    )
    .unwrap();
    let client = root.join("driver");
    let built = Command::new(compilers[0])
        .args(["-std=gnu11", "-O2"])
        .arg(&driver)
        .args(["-ldl", "-o"])
        .arg(&client)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    for enabled in [false, true] {
        let custom = model.replace("MECHANISM", if enabled { mechanism } else { "" });
        std::fs::write(
            specs.join("x86-64-gcc.cspec"),
            base.replace("</compiler_spec>", &(custom + "</compiler_spec>")),
        )
        .unwrap();
        let (output, error, status) = common::run_kuna(&[
            "decompile-all",
            original.to_str().unwrap(),
            "--functions",
            "sample",
            "--sleighpath",
            specs.to_str().unwrap(),
            "--json",
            "--option",
            "callmechanisms",
            "on",
            "--assert",
            "prototype sample unsigned sample(unsigned *values)",
            "--assert",
            "prototype poke void poke_effect poke(unsigned *values)",
        ]);
        assert_eq!(status, 0, "{error}");
        let document: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(document["functions"][0]["error"].is_null(), "{output}");
        let code = document["functions"][0]["code"].as_str().unwrap();
        let regenerated = root.join("regenerated.c");
        std::fs::write(
            &regenerated,
            format!(
                "#define poke_effect\nvoid poke(unsigned *values) {{ (void)values; }}\n{code}\n"
            ),
        )
        .unwrap();
        for compiler in &compilers {
            for level in ["-O0", "-O2"] {
                let binary = root.join(format!("regenerated-{enabled}-{compiler}-{level}.so"));
                let built = Command::new(compiler)
                    .args([
                        level,
                        "-shared",
                        "-fPIC",
                        "-Werror=implicit-function-declaration",
                    ])
                    .arg(&regenerated)
                    .arg("-o")
                    .arg(&binary)
                    .output()
                    .unwrap();
                assert!(
                    built.status.success(),
                    "{code}\n{}",
                    String::from_utf8_lossy(&built.stderr)
                );
                let run = Command::new(&client)
                    .arg(&original)
                    .arg(&binary)
                    .output()
                    .unwrap();
                assert_eq!(
                    run.status.code(),
                    Some(if enabled { 0 } else { 1 }),
                    "enabled={enabled}, {compiler} {level}\n{code}\n{}",
                    String::from_utf8_lossy(&run.stderr)
                );
            }
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn dynamic_entry_mechanisms_without_a_body_do_not_fail_the_function() {
    for (target, bytes, expected) in [
        ("JVM:BE:32:default", &[0x04u8, 0xac][..], "return 1;"),
        ("Dalvik:LE:32:DEX_Base", &[0x0e, 0x00][..], "return;"),
    ] {
        let image = common::scratch_file("dynamic-entry-mechanism", "bin");
        std::fs::write(&image, bytes).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", common::repo_root().join("specs"))
            .args([
                "decompile",
                image.to_str().unwrap(),
                "0x1000",
                "--raw-image",
            ])
            .args(["--option", "callmechanisms", "on"])
            .args(["--target", target, "--base", "0x1000"])
            .output()
            .unwrap();
        let _ = std::fs::remove_file(&image);
        let code = String::from_utf8_lossy(&output.stdout);
        assert!(
            output.status.success() && code.contains(expected),
            "{target}\n{code}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// The stock ELFv1 `r2Save` store to `0x28(r1)` lands on a live ELFv2 local.
#[test]
fn default_off_keeps_a_local_in_the_elfv1_toc_slot_of_an_elfv2_frame() {
    let binary = common::fixture("callmech_tocslot_ppc64le.o");
    let returned = |value: &str| {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", common::repo_root().join("specs"))
            .args([
                "decompile",
                &binary,
                "two_calls",
                "--option",
                "callmechanisms",
                value,
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let code = String::from_utf8(output.stdout).unwrap();
        let product = code
            .lines()
            .find_map(|line| line.trim().strip_suffix(" = a0 * 9;"))
            .unwrap_or_else(|| panic!("{code}"))
            .to_owned();
        let kept = code
            .lines()
            .any(|line| line.trim().starts_with("return ") && line.contains(product.as_str()));
        (kept, code)
    };
    let (kept, code) = returned("off");
    assert!(kept && !code.contains("r2save"), "{code}");
    let (kept, code) = returned("on");
    assert!(!kept && code.contains("r2save"), "{code}");
}
