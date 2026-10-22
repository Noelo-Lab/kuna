//! Declared pointer values stored in integer frame members remain valid C.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::{Object, ObjectSection};
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn pe32_array_pointer_slot() {
    check(true, false, false);
}
#[test]
fn elf64_array_derived_pointer_slot_roundtrip() {
    check_derived(false);
}
#[test]
fn elf64_array_derived_pointer_slot_with_views_roundtrip() {
    check_derived(true);
}
#[test]
fn elf64_array_pointer_slot_roundtrip() {
    check(false, false, false);
}
#[test]
fn pe32_array_pointer_slot_with_views() {
    check(true, true, false);
}
#[test]
fn elf64_array_pointer_slot_with_views_roundtrip() {
    check(false, true, false);
}

#[test]
fn pe32_record_pointer_slot() {
    check(true, false, true);
}
#[test]
fn elf64_record_pointer_slot_roundtrip() {
    check(false, false, true);
}

#[test]
fn record_declared_source_keeps_cast() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("pointer-slot-record-source", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let header = "typedef struct W { unsigned char *p; } W;\nextern void g(void *);\n";
    std::fs::write(
        &source,
        format!("{header}long load(W w) {{ long x = (long)w.p; g(&x); return x; }}\n"),
    )
    .unwrap();
    let object = root.join("sample.o");
    process::required_output(
        Command::new("gcc")
            .args(["-g", "-O2", "-fno-stack-protector", "-fcf-protection=none", "-c"])
            .arg(&source)
            .arg("-o")
            .arg(&object),
    );
    let (body, err, status) = common::run_kuna(&["decompile", object.to_str().unwrap(), "load"]);
    assert_eq!(status, 0, "{err}");
    let generated = root.join("generated.c");
    std::fs::write(&generated, format!("{header}{body}")).unwrap();
    process::required_output(
        Command::new("gcc")
            .args(["-std=gnu11", "-Werror=int-conversion", "-c"])
            .arg(&generated)
            .arg("-o")
            .arg(root.join("generated.o")),
    );
}

fn check(windows: bool, views: bool, record: bool) {
    run(windows, views, record, false);
}

fn check_derived(views: bool) {
    run(false, views, false, true);
}

fn run(windows: bool, views: bool, record: bool, derived: bool) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("pointer-slot-cast", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let mov = if derived { "lea 1(%rdi),%rax" } else { "mov %rdi,%rax" };
    std::fs::write(&source, format!("#define MOVE \"{mov}\"\n") + r#"
typedef __UINTPTR_TYPE__ uintptr_t;
#ifdef _WIN32
#define API __declspec(dllexport)
#else
#define API __attribute__((visibility("default")))
#endif
API __attribute__((noinline)) unsigned consume(void *slot) {
 uintptr_t value;__builtin_memcpy(&value,slot,sizeof(value));return *(unsigned char*)value;
}
API __attribute__((naked)) unsigned emit(unsigned char *value) {
#ifdef _WIN32
 __asm__("sub $16,%esp;mov 20(%esp),%eax;mov %eax,(%esp);mov %esp,%eax;push %eax;call _consume;add $20,%esp;ret");
#else
 __asm__("sub $24,%rsp;" MOVE ";mov %rax,(%rsp);mov %rsp,%rdi;call consume;add $24,%rsp;ret");
#endif
}
"#).unwrap();
    let binary = root.join(if windows { "sample.dll" } else { "sample.so" });
    let mut build = Command::new("clang");
    build.args(["-O2", "-shared", "-nostdlib"]);
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
    let entry = address(b"emit");
    let section = image
        .sections()
        .find(|s| s.address() <= entry && entry < s.address() + s.size())
        .unwrap();
    let bytes = section.data().unwrap();
    let start = (entry - section.address()) as usize;
    let code = &bytes[start..bytes.len().min(start + 48)];
    let instruction: &[u8] = if windows {
        &[0x8b, 0x44, 0x24, 0x14]
    } else if derived {
        &[0x48, 0x8d, 0x47, 0x01]
    } else {
        &[0x48, 0x89, 0xf8]
    };
    let pc = entry
        + code
            .windows(instruction.len())
            .position(|b| b == instruction)
            .unwrap() as u64;
    let (word, count, offset, register) = if windows {
        ("unsigned int", 4, "s0xfffffff0", "EAX")
    } else {
        ("unsigned long", 2, "s0xffffffffffffffe8", "RAX")
    };
    let declaration = format!(
        "typedef struct Frame {{{word} slot;unsigned char tail[{}];}} Frame;",
        if windows { 12 } else { 8 }
    );
    let mapping = if record {
        "Frame frame".to_string()
    } else {
        format!("{word} frame[{count}]")
    };
    let body_file = root.join("body.c");
    let script = format!("load file \"{}\"\nread symbols\noption ctypes on\nparse line {declaration}\noption stackviews {}\nmap prototype 0x{entry:x} unsigned int emit(unsigned char *value)\nmap prototype 0x{:x} unsigned int consume(void *slot)\nload addr 0x{entry:x}\nmap address {offset} {mapping}\ntype varnode %{register}(0x{pc:x}) unsigned char *address\ndecompile\nopenfile write \"{}\"\nprint C\nclosefile\nquit\n",binary.display(),if views {"on"}else{"off"},address(b"consume"),body_file.display());
    render(&script);
    let body = std::fs::read_to_string(&body_file).unwrap();
    std::fs::write(
        &body_file,
        format!("{declaration}\nunsigned int consume(void *);\n{body}"),
    )
    .unwrap();
    for opt in ["-O0", "-O2"] {
        let object = root.join(format!("generated{opt}.o"));
        let mut compile = Command::new("clang");
        compile.args([
            "-std=gnu11",
            "-Werror",
            opt,
            "-Demit=generated_emit",
            "-D_emit=generated_emit",
            "-D_consume=consume",
        ]);
        if windows {
            compile.arg("--target=i686-w64-windows-gnu");
        }
        process::required_output(compile.arg("-c").arg(&body_file).arg("-o").arg(&object));
        if !windows {
            let driver = root.join("driver.c");
            std::fs::write(&driver,format!("#define OFFSET {}\n", derived as u8) + r#"
#include <stdint.h>
#include <dlfcn.h>
unsigned generated_emit(unsigned char *);
unsigned consume(void *slot){uintptr_t value;__builtin_memcpy(&value,slot,sizeof(value));return *(unsigned char*)value;}
int main(int argc,char **argv){
 if(argc!=2)return 1;void *module=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL);if(!module)return 2;
 unsigned (*native)(unsigned char*)=dlsym(module,"emit");if(!native)return 3;
 unsigned char bytes[257];for(unsigned i=0;i<257;i++)bytes[i]=(unsigned char)(i*37);
 for(unsigned i=0;i<4096;i++){unsigned char *p=bytes+i%256;if(native(p)!=OFFSET[p]||generated_emit(p)!=native(p))return 4;}
 return 0;
}
"#).unwrap();
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
