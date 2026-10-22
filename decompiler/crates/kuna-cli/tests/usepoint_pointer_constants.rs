//! Explicit register pointer declarations survive propagation through integer storage.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::{Object, ObjectSection, ObjectSymbol};
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn pe32_pointer_constant() {
    check(true, false);
}
#[test]
fn elf64_pointer_constant_roundtrip() {
    check(false, false);
}
#[test]
fn pe32_pointer_constant_with_entry_prefix() {
    check(true, true);
}
fn check(windows: bool, entry_prefix: bool) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("usepoint-pointer-constant", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    std::fs::write(&source,r#"
typedef __UINTPTR_TYPE__ uintptr_t;
#ifdef _WIN32
#define API __declspec(dllexport)
#else
#define API __attribute__((visibility("default")))
#endif
unsigned char blob[16]={2,4,6,11,15,19};
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
    if entry_prefix {
        build.arg("-fcf-protection=branch");
    }
    if windows {
        build.args(["--target=i686-w64-windows-gnu", "-Wl,--entry,0"]);
    } else {
        build.args(["-fPIC", "-Wl,-Bsymbolic"]);
    }
    process::required_output(build.arg(&source).arg("-o").arg(&binary));
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let exports = image.exports().unwrap();
    // Keep storage out of the PE export directory: exported-data classification
    // has its own regression fixture and must not obscure the pointer assertion.
    let blob_address = image
        .symbols()
        .find(|symbol| {
            symbol
                .name()
                .is_ok_and(|name| name == "blob" || name == "_blob")
        })
        .expect("global storage symbol")
        .address();
    let address = |name: &[u8]| exports.iter().find(|e| e.name() == name).unwrap().address();
    let word = if windows {
        "unsigned int"
    } else {
        "unsigned long long"
    };
    let register = if windows { "ECX" } else { "RCX" };
    let entry = address(b"emit");
    let section = image
        .sections()
        .find(|section| section.address() <= entry && entry < section.address() + section.size())
        .unwrap();
    let section_bytes = section.data().unwrap();
    let start = (entry - section.address()) as usize;
    let code = &section_bytes[start..section_bytes.len().min(start + 32)];
    // Compiler-inserted entry instructions must not shift the declaration to
    // another write. Locate the authored address instruction in the image.
    let offset = if windows {
        let mut instruction = vec![0xb9];
        instruction.extend_from_slice(&((blob_address + 3) as u32).to_le_bytes());
        code.windows(instruction.len())
            .position(|bytes| bytes == instruction)
    } else {
        code.windows(3)
            .position(|bytes| bytes == [0x48, 0x8d, 0x0d])
    }
    .expect("authored address instruction");
    let pc = entry + offset as u64;
    let c = root.join("body.c");
    let script=format!("load file \"{}\"\nread symbols\noption ctypes on\nmap prototype 0x{:x} unsigned int emit(volatile {} *slot)\nmap address 0x{:x} unsigned char blob[16]\nload addr 0x{:x}\ntype varnode %{}(0x{:x}) unsigned char *address\ndecompile\nopenfile write \"{}\"\nprint C\nclosefile\nquit\n",binary.display(),address(b"emit"),word,blob_address,address(b"emit"),register,pc,c.display());
    render(&script);
    let body = std::fs::read_to_string(&c).unwrap();
    assert!(
        body.contains("blob"),
        "declared address at {pc:x} lost; bytes {code:02x?}: {body}"
    );
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

#[test]
fn pe32_dynamic_interior_pointer() {
    check_dynamic(true);
}
#[test]
fn elf64_dynamic_interior_pointer_roundtrip() {
    check_dynamic(false);
}
fn check_dynamic(windows: bool) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("usepoint-pointer-dynamic", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    std::fs::write(&source,r#"
typedef __UINTPTR_TYPE__ uintptr_t;
#ifdef _WIN32
#define API __declspec(dllexport)
#else
#define API __attribute__((visibility("default")))
#endif
unsigned char blob[512]={2,4,6,11,15,19};
API __attribute__((naked)) unsigned int emit(uintptr_t *slot,unsigned int index) {
#ifdef _WIN32
__asm__("movl 4(%esp),%ecx\nmovl 8(%esp),%edx\nleal 0(,%edx,4),%eax\nleal _blob+128(%eax),%edx\nmovl %edx,(%ecx)\nxorl %eax,%eax\nretl\n");
#else
__asm__("movl %esi,%esi\nleaq blob+128(%rip),%rcx\nleaq (%rcx,%rsi,4),%rcx\nmovq %rcx,(%rdi)\nxorl %eax,%eax\nretq\n");
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
    // Keep storage out of the PE export directory: exported-data classification
    // has its own regression fixture and must not obscure the pointer assertion.
    let blob_address = image
        .symbols()
        .find(|symbol| {
            symbol
                .name()
                .is_ok_and(|name| name == "blob" || name == "_blob")
        })
        .expect("global storage symbol")
        .address();
    let address = |name: &[u8]| exports.iter().find(|e| e.name() == name).unwrap().address();
    let word = if windows {
        "unsigned int"
    } else {
        "unsigned long long"
    };
    let register = if windows { "EDX" } else { "RCX" };
    let entry = address(b"emit");
    let section = image
        .sections()
        .find(|section| section.address() <= entry && entry < section.address() + section.size())
        .unwrap();
    let section_bytes = section.data().unwrap();
    let start = (entry - section.address()) as usize;
    let code = &section_bytes[start..section_bytes.len().min(start + 32)];
    // Compiler-inserted entry instructions must not shift the declaration to
    // another write. Locate the authored address instruction in the image.
    let offset = if windows {
        let mut instruction = vec![0x8d, 0x90];
        instruction.extend_from_slice(&((blob_address + 128) as u32).to_le_bytes());
        code.windows(instruction.len())
            .position(|bytes| bytes == instruction)
    } else {
        code.windows(4)
            .position(|bytes| bytes == [0x48, 0x8d, 0x0c, 0xb1])
    }
    .expect("authored address instruction");
    let pc = entry + offset as u64;
    let c = root.join("body.c");
    let script=format!("load file \"{}\"\nread symbols\noption ctypes on\nmap prototype 0x{:x} unsigned int emit({} *slot,unsigned int index)\nmap address 0x{:x} unsigned char blob[512]\nload addr 0x{:x}\ntype varnode %{}(0x{:x}) unsigned char *address\ndecompile\nopenfile write \"{}\"\nprint C\nclosefile\nquit\n",binary.display(),address(b"emit"),word,blob_address,address(b"emit"),register,pc,c.display());
    render(&script);
    let body = std::fs::read_to_string(&c).unwrap();
    assert!(
        body.contains("blob"),
        "declared address at {pc:x} lost; bytes {code:02x?}: {body}"
    );
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
    std::fs::write(&c, format!("extern unsigned char blob[512];\n{body}")).unwrap();
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
unsigned char blob[512]={2,4,6,11,15,19};
unsigned int generated_emit(uintptr_t *,unsigned int);
int main(int argc,char **argv) {
 if(argc!=2)return 1;
 void *module=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL);if(!module)return 2;
 unsigned int (*native)(uintptr_t *,unsigned int)=dlsym(module,"emit");
 unsigned char *original=dlsym(module,"blob");if(!native||!original)return 3;
 for(unsigned int i=0;i<4096;i++) {
  uintptr_t a=0,b=0;unsigned index=i%96;
  if(native(&a,index)!=0||generated_emit(&b,index)!=0)return 4;
  if(a!=(uintptr_t)(original+128+index*4)||b!=(uintptr_t)(blob+128+index*4))return 5;
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
