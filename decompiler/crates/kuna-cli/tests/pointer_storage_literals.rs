//! Pointer stores into explicitly mapped byte storage remain compilable C.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn pe32_pointer_initializer_roundtrip() {
    check(true);
}
#[test]
fn elf64_pointer_initializer_roundtrip() {
    check(false);
}
fn check(windows: bool) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("pointer-storage-literal", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    std::fs::write(&source,r#"
typedef __UINTPTR_TYPE__ uintptr_t;
typedef struct Record {unsigned int length;void *pointer;} Record;
#ifdef _WIN32
#define API __declspec(dllexport) __attribute__((noinline))
#else
#define API __attribute__((visibility("default"),noinline))
#endif
API unsigned int consume(Record *r){return r->length+((uintptr_t)r->pointer&255);}
API __attribute__((naked)) unsigned int emit(char *payload){
#ifdef _WIN32
__asm__("pushl %ebp\nmovl %esp,%ebp\nsubl $8,%esp\nmovl 8(%ebp),%eax\nmovl $5,-8(%ebp)\nmovl %eax,-4(%ebp)\nleal -8(%ebp),%eax\npushl %eax\ncalll _consume\naddl $4,%esp\nmovl %ebp,%esp\npopl %ebp\nretl\n");
#else
__asm__("pushq %rbp\nmovq %rsp,%rbp\nsubq $16,%rsp\nmovl $5,-16(%rbp)\nmovq %rdi,-8(%rbp)\nleaq -16(%rbp),%rdi\ncall consume\nmovq %rbp,%rsp\npopq %rbp\nretq\n");
#endif
}
"#).unwrap();
    let target = windows.then_some("--target=i686-w64-windows-gnu");
    let binary = root.join(if windows { "sample.dll" } else { "sample.so" });
    let mut build = Command::new("clang");
    build.args(["-O0", "-shared", "-nostdlib"]);
    if let Some(target) = target {
        build.args([target, "-Wl,--entry,0"]);
    } else {
        build.args(["-fPIC", "-Wl,-Bsymbolic"]);
    }
    process::required_output(build.arg(&source).arg("-o").arg(&binary));
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let exports = image.exports().unwrap();
    let address = |name: &[u8]| exports.iter().find(|e| e.name() == name).unwrap().address();
    let c = root.join("body.c");
    // The authored assembly places only the Record at this fixed frame slot.
    let size = if windows { 8 } else { 16 };
    let word = if windows {
        "s0xfffffff4"
    } else {
        "s0xffffffffffffffe8"
    };
    let script=format!("load file \"{}\"\nread symbols\noption ctypes on\noption partialaccess on\noption stackviews on\nparse line typedef struct Record {{unsigned int length;void *pointer;}} Record;\nmap prototype 0x{:x} unsigned int emit(char *payload);\nmap prototype 0x{:x} unsigned int consume(Record *r);\nload addr 0x{:x}\nmap address {} unsigned char frame[{}]\ndecompile\nopenfile write \"{}\"\nprint C\nclosefile\nquit\n",binary.display(),address(b"emit"),address(b"consume"),address(b"emit"),word,size,c.display());
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
    let transcript = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!transcript.contains("Execution error:"), "{transcript}");
    let body = std::fs::read_to_string(&c).unwrap();
    assert!(body.contains("__builtin_memcpy"), "{body}");
    let generated = root.join("generated.c");
    std::fs::write(&generated,format!("typedef struct Record {{unsigned int length;void *pointer;}} Record;unsigned int consume(Record*);unsigned int _consume(Record*);\n{body}")).unwrap();
    let driver = root.join("driver.c");
    std::fs::write(&driver,r#"
#include <stdint.h>
typedef struct Record {unsigned int length;void *pointer;} Record;
unsigned int native_emit(char*);unsigned int regenerated_emit(char*);unsigned int native_consume(Record*);
unsigned int consume(Record*r){return native_consume(r);}unsigned int regenerated_consume(Record*r){return native_consume(r);}
int main(void){char bytes[4096];for(unsigned i=0;i<sizeof(bytes);i++){char *p=bytes+i;unsigned want=5+((uintptr_t)p&255);if(native_emit(p)!=want||regenerated_emit(p)!=want)return 1;}return 0;}
"#).unwrap();
    let native = root.join("native.o");
    let mut build = Command::new("clang");
    if let Some(t) = target {
        build.arg(t);
    }
    build
        .args([
            "-O0",
            "-Demit=native_emit",
            "-Dconsume=native_consume",
            "-c",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&native);
    process::required_output(&mut build);
    for opt in ["-O0", "-O2"] {
        let object = root.join("generated.o");
        let mut build = Command::new("clang");
        if let Some(t) = target {
            build.arg(t);
        }
        build
            .args([
                "-std=gnu11",
                "-Werror",
                opt,
                "-Demit=regenerated_emit",
                "-D_emit=regenerated_emit",
                "-Dconsume=regenerated_consume",
                "-D_consume=regenerated_consume",
                "-c",
            ])
            .arg(&generated)
            .arg("-o")
            .arg(&object);
        process::required_output(&mut build);
        if !windows {
            let executable = root.join("check");
            process::required_output(
                Command::new("clang")
                    .arg(opt)
                    .arg(&driver)
                    .arg(&native)
                    .arg(&object)
                    .arg("-o")
                    .arg(&executable),
            );
            process::required_output(&mut Command::new(executable));
        }
    }
}
