use crate::common;
use std::{fs, process::Command};

fn check(target: &str, windows: bool) {
    if Command::new("clang").arg("--version").output().is_err() {
        return;
    }
    let dir = common::scratch_file("implied-word-cast", "dir");
    fs::create_dir_all(&dir).unwrap();
    let source = dir.join("fixture.c");
    fs::write(&source, r#"
typedef union block { float real[32]; unsigned int integer[32]; } block;
block items;
__attribute__((naked)) unsigned int transfer(unsigned int index) {
#ifdef _WIN32
 __asm__("movl 4(%esp),%eax\n shll $2,%eax\n movl _items(%eax),%edx\n movl %edx,_items+64\n flds _items+64\n fstps _items+68\n movl _items+64,%eax\n retl\n");
#else
 __asm__("movl %edi,%eax\n leaq items(%rip),%rcx\n movl (%rcx,%rax,4),%edx\n movl %edx,64(%rcx)\n flds 64(%rcx)\n fstps 68(%rcx)\n movl 64(%rcx),%eax\n retq\n");
#endif
}
"#).unwrap();
    let object = dir.join("fixture.o");
    let result = Command::new("clang")
        .args(["-target", target, "-O1", "-fno-pic", "-fno-pie", "-c"])
        .arg(&source)
        .arg("-o")
        .arg(&object)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let export = dir.join("export");
    let (_, stderr, status) = common::run_kuna(&[
        "decompile-project",
        object.to_str().unwrap(),
        "-o",
        export.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{stderr}");
    let readme = fs::read_to_string(export.join("README.md")).unwrap();
    let row = readme
        .lines()
        .find(|line| line.starts_with("| `.bss` |"))
        .unwrap();
    let address = row.split('|').nth(2).unwrap().trim().trim_matches('`');
    let data = format!("data {address} block items");
    let name = if windows { "_transfer" } else { "transfer" };
    let prototype = format!(
        "prototype {name} unsigned int {}transfer(unsigned int index)",
        if windows { "__cdecl " } else { "" }
    );
    let (_, stderr, status) = common::run_kuna(&[
        "decompile-project",
        object.to_str().unwrap(),
        "-o",
        export.to_str().unwrap(),
        "--assert",
        "typedef union block {float real[32];unsigned int integer[32];} block;",
        "--assert",
        &data,
        "--assert",
        &prototype,
        "--assert-strict",
        "--option",
        "indexaliasguard",
        "full",
        "--option",
        "stackviews",
        "on",
    ]);
    assert_eq!(status, 0, "{stderr}");
    let generated = fs::read_to_string(export.join("fixture.o.c")).unwrap();
    assert!(
        generated.contains("union { unsigned int from; float to; }"),
        "{target}: {generated}"
    );
    let wrapper = dir.join("regenerated.c");
    fs::write(
        &wrapper,
        format!(
            "#define {name} regenerated_transfer\n#include \"{}\"\n",
            export.join("fixture.o.c").display()
        ),
    )
    .unwrap();
    let driver = dir.join("driver.c");
    fs::write(
        &driver,
        r#"
#include "export/fixture.o.h"
unsigned int transfer(unsigned int);
unsigned int regenerated_transfer(unsigned int);
int main(void) {
 for(unsigned i=1;i<=4096;i++) {
  unsigned index=i%16, bits=i*2654435761u;
  items.integer[index]=bits;
  unsigned expected=transfer(index);
  items.integer[index]=bits;
  unsigned actual=regenerated_transfer(index);
  if(expected!=bits || actual!=expected || items.integer[16]!=bits) return 1;
 }
 return 0;
}
"#,
    )
    .unwrap();
    for opt in ["-O0", "-O2"] {
        let output = dir.join(format!("generated{opt}.o"));
        let compiled = Command::new("clang")
            .args(["-target", target, opt, "-Werror", "-c"])
            .arg(&wrapper)
            .arg("-o")
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            compiled.status.success(),
            "{}",
            String::from_utf8_lossy(&compiled.stderr)
        );
        if !windows && cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            let executable = dir.join(format!("roundtrip{opt}"));
            let linked = Command::new("clang")
                .args(["-target", target, opt, "-no-pie"])
                .arg(&object)
                .arg(&output)
                .arg(&driver)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                linked.status.success(),
                "{}",
                String::from_utf8_lossy(&linked.stderr)
            );
            assert!(
                Command::new(&executable).status().unwrap().success(),
                "{opt}: native word-copy mismatch"
            );
        }
    }
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn implied_word_cast_keeps_float_member_bits_pe32() {
    check("i686-w64-windows-gnu", true);
}

#[test]
fn implied_word_cast_keeps_float_member_bits_elf64() {
    check("x86_64-unknown-linux-gnu", false);
}
