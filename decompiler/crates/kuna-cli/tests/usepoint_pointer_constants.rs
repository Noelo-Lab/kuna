//! Explicit register pointer declarations survive constant propagation.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn pe32_pointer_constant() {
    check(true);
}
#[test]
fn elf64_pointer_constant_roundtrip() {
    check(false);
}
fn check(windows: bool) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("usepoint-pointer-constant", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    std::fs::write(&source,r#"
#include <stdint.h>
#ifdef _WIN32
#define API __declspec(dllexport)
#else
#define API __attribute__((visibility("default")))
#endif
API unsigned char blob[16]={2,4,6,11,15,19};
API __attribute__((naked)) unsigned int emit(volatile uintptr_t *slot) {
#ifdef _WIN32
__asm__("movl 4(%esp),%eax\nmovl $_blob+3,%ecx\nmovl %ecx,(%eax)\nmovl (%eax),%eax\nmovzbl (%eax),%eax\nretl\n");
#else
__asm__("leaq blob+3(%rip),%rcx\nmovq %rcx,(%rdi)\nmovq (%rdi),%rax\nmovzbl (%rax),%eax\nretq\n");
#endif
}
"#).unwrap();
    let binary = root.join(if windows { "sample.dll" } else { "sample.so" });
    let mut build = Command::new("clang");
    build.args(["-O0", "-shared", "-nostdlib"]);
    if windows {
        build.args(["--target=i686-w64-windows-gnu", "-Wl,--entry,0"]);
    } else {
        build.args(["-fPIC", "-Wl,-Bsymbolic"]);
    }
    process::required_output(build.arg(&source).arg("-o").arg(&binary));
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let exports = image.exports().unwrap();
    let address = |name: &[u8]| exports.iter().find(|e| e.name() == name).unwrap().address();
    let word = if windows {
        "unsigned int"
    } else {
        "unsigned long long"
    };
    let register = if windows { "ECX" } else { "RCX" };
    let pc = address(b"emit") + if windows { 4 } else { 0 };
    let c = root.join("body.c");
    let script=format!("load file \"{}\"\nread symbols\noption ctypes on\nmap prototype 0x{:x} unsigned int emit(volatile {} *slot)\nmap address 0x{:x} unsigned char blob[16]\nload addr 0x{:x}\ntype varnode %{}(0x{:x}) unsigned char *address\ndecompile\nopenfile write \"{}\"\nprint C\nclosefile\nquit\n",binary.display(),address(b"emit"),word,address(b"blob"),address(b"emit"),register,pc,c.display());
    render(&script);
    let body = std::fs::read_to_string(&c).unwrap();
    assert!(body.contains("blob"), "declared address lost: {body}");
    let control = root.join("integer-control.c");
    let untyped = script
        .lines()
        .filter(|line| !line.starts_with("type varnode "))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    render(&untyped.replace(c.to_str().unwrap(), control.to_str().unwrap()));
    let control_body = std::fs::read_to_string(control).unwrap();
    assert!(
        !control_body.contains("blob"),
        "unasserted integer changed: {control_body}"
    );
    std::fs::write(&c, format!("extern unsigned char blob[16];\n{body}")).unwrap();
    for opt in ["-O0", "-O2"] {
        let object = root.join(format!("generated{opt}.o"));
        let mut compile = Command::new("clang");
        compile.args(["-std=gnu11", "-Werror", opt]);
        if windows {
            compile.arg("--target=i686-w64-windows-gnu");
        }
        process::required_output(
            compile
                .arg("-Demit=generated_emit")
                .arg("-c")
                .arg(&c)
                .arg("-o")
                .arg(&object),
        );
        if !windows {
            let driver = root.join("driver.c");
            std::fs::write(
                &driver,
                r#"
#include <stdint.h>
#include <dlfcn.h>
unsigned char blob[16]={2,4,6,11,15,19};
unsigned int generated_emit(volatile uintptr_t *);
int main(int argc,char **argv) {
 if(argc!=2)return 1;
 void *module=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL);if(!module)return 2;
 unsigned int (*native)(volatile uintptr_t *)=dlsym(module,"emit");
 unsigned char *original=dlsym(module,"blob");if(!native||!original)return 3;
 for(unsigned int i=0;i<4096;i++) {
  volatile uintptr_t a=0,b=0;
  if(native(&a)!=11||generated_emit(&b)!=11)return 4;
  if(a!=(uintptr_t)(original+3)||b!=(uintptr_t)(blob+3))return 5;
 }
 return 0;
}
"#,
            )
            .unwrap();
            let exe = root.join(format!("compare{opt}"));
            process::required_output(
                Command::new("clang")
                    .arg(opt)
                    .arg(&driver)
                    .arg(&object)
                    .arg("-ldl")
                    .arg("-o")
                    .arg(&exe),
            );
            process::required_output(Command::new(&exe).arg(&binary));
        }
    }
}

fn render(script: &str) {
    let engine = std::env::var_os("KUNA_DECOMP_DBG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_BIN_EXE_kuna"))
                .parent()
                .unwrap()
                .join("decomp_dbg")
        });
    let mut child = Command::new(engine)
        .arg("-s")
        .arg(common::repo_root().join("specs"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
