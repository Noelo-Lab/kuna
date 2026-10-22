use crate::common;
use std::{fs, process::Command};

fn check(target: &str, windows: bool, integer_member: bool) {
    if Command::new("clang").arg("--version").output().is_err() {
        return;
    }
    let dir = common::scratch_file("unaligned-word-load", "dir");
    fs::create_dir_all(&dir).unwrap();
    let source = dir.join("fixture.c");
    fs::write(&source, r#"
typedef union block { float real[32]; unsigned int integer[32]; } block;
block items;
__attribute__((naked)) unsigned int transfer(unsigned int index) {
#ifdef _WIN32
 __asm__("movl 4(%esp),%eax\n shll $2,%eax\n movl _items+1(%eax),%edx\n movl %edx,_items+64\n flds _items+64\n fstps _items+68\n movl _items+64,%edx\n movl %edx,_items+5(%eax)\n movl _items+64,%eax\n retl\n");
#else
 __asm__("movl %edi,%eax\n leaq items(%rip),%rcx\n movl 1(%rcx,%rax,4),%edx\n movl %edx,64(%rcx)\n flds 64(%rcx)\n fstps 68(%rcx)\n movl 64(%rcx),%edx\n movl %edx,5(%rcx,%rax,4)\n movl 64(%rcx),%eax\n retq\n");
#endif
}
__attribute__((naked)) unsigned int scatter(unsigned int index) {
#ifdef _WIN32
 __asm__("movl 4(%esp),%eax\n movl %eax,%ecx\n shll $2,%eax\n testl %ecx,%ecx\n jz 2f\n 1: incl _items+64\n movl _items+64,%edx\n movl %edx,_items+1(%eax)\n decl %ecx\n jnz 1b\n 2: movl _items+64,%eax\n retl\n");
#else
 __asm__("movl %edi,%eax\n leaq items(%rip),%rcx\n movl %eax,%esi\n testl %esi,%esi\n jz 2f\n 1: incl 64(%rcx)\n movl 64(%rcx),%edx\n movl %edx,1(%rcx,%rax,4)\n decl %esi\n jnz 1b\n 2: movl 64(%rcx),%eax\n retq\n");
#endif
}
__attribute__((naked)) unsigned int choose(unsigned int index, unsigned int odd) {
#ifdef _WIN32
 __asm__("movl 4(%esp),%eax\n shll $2,%eax\n testl $1,8(%esp)\n jz 1f\n movl _items+1(%eax),%edx\n jmp 2f\n 1: movl _items+3(%eax),%edx\n 2: movl %edx,_items+64\n flds _items+64\n fstps _items+68\n movl _items+64,%eax\n retl\n");
#else
 __asm__("movl %edi,%eax\n leaq items(%rip),%rcx\n testl $1,%esi\n jz 1f\n movl 1(%rcx,%rax,4),%edx\n jmp 2f\n 1: movl 3(%rcx,%rax,4),%edx\n 2: movl %edx,64(%rcx)\n flds 64(%rcx)\n fstps 68(%rcx)\n movl 64(%rcx),%eax\n retq\n");
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
    let scatter_prototype = format!(
        "prototype {} unsigned int {}scatter(unsigned int index)",
        if windows { "_scatter" } else { "scatter" },
        if windows { "__cdecl " } else { "" }
    );
    let choose_prototype = format!(
        "prototype {} unsigned int {}choose(unsigned int index, unsigned int odd)",
        if windows { "_choose" } else { "choose" },
        if windows { "__cdecl " } else { "" }
    );
    let (_, stderr, status) = common::run_kuna(&[
        "decompile-project",
        object.to_str().unwrap(),
        "-o",
        export.to_str().unwrap(),
        "--assert",
        &scatter_prototype,
        "--assert",
        &choose_prototype,
        "--assert",
        if integer_member {
            "typedef union block {float real[32];unsigned int integer[32];} block;"
        } else {
            "typedef struct block {float real[32];} block;"
        },
        "--assert",
        &data,
        "--assert",
        &prototype,
        "--assert-strict",
        "--option",
        "nanignore",
        "none",
        "--option",
        "indexaliasguard",
        "full",
        "--option",
        "stackviews",
        "on",
    ]);
    assert_eq!(status, 0, "{stderr}");
    let generated = fs::read_to_string(export.join("fixture.o.c")).unwrap();
    let wrapper = dir.join("regenerated.c");
    fs::write(
        &wrapper,
        format!(
            "#define {name} regenerated_transfer\n#define {} regenerated_scatter\n#define {} regenerated_choose\n#include \"{}\"\n",
            if windows { "_scatter" } else { "scatter" },
            if windows { "_choose" } else { "choose" },
            export.join("fixture.o.c").display()
        ),
    )
    .unwrap();
    let driver = dir.join("driver.c");
    fs::write(
        &driver,
        r#"
#include "export/fixture.o.h"
#include <string.h>
#include <stdio.h>
unsigned int transfer(unsigned int);
unsigned int regenerated_transfer(unsigned int);
unsigned int scatter(unsigned int);
unsigned int regenerated_scatter(unsigned int);
unsigned int choose(unsigned int, unsigned int);
unsigned int regenerated_choose(unsigned int, unsigned int);
int main(void) {
 for(unsigned i=1;i<=4096;i++) {
  unsigned index=i%12, bits=i==1 ? 1 : i*2654435761u;
  /* Keep the independent x87 signaling-NaN conversion out of this word-copy case. */
  if((bits & 0x7f800000u)==0x7f800000u) bits ^= 0x00800000u;
  memcpy((unsigned char *)&items + index*4 + 1, &bits, sizeof bits);
  block initial=items;
  unsigned expected=transfer(index);
  block expected_state=items;
  items=initial;
  unsigned actual=regenerated_transfer(index);
  unsigned stored; memcpy(&stored, (unsigned char *)&items+64, sizeof stored);
  if(expected!=bits || actual!=expected || stored!=bits) return 1;
  if(memcmp(&items, &expected_state, sizeof items)) {
   fprintf(stderr,"case=%u bits=%08x full-state mismatch\n",i,bits);
   return 2;
  }
  initial=items;
  expected=scatter(index);
  expected_state=items;
  items=initial;
  actual=regenerated_scatter(index);
  if(expected!=bits+index || actual!=expected || memcmp(&items,&expected_state,sizeof items)) return 3;
  unsigned odd=i&1, loaded;
  memcpy(&loaded, (unsigned char *)&items + index*4 + (odd ? 1 : 3), sizeof loaded);
  if((loaded & 0x7f800000u)==0x7f800000u) continue;
  initial=items;
  expected=choose(index, odd);
  expected_state=items;
  items=initial;
  actual=regenerated_choose(index, odd);
  if(expected!=loaded || actual!=expected || memcmp(&items,&expected_state,sizeof items)) return 4;
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
    assert!(
        generated.contains("__builtin_memcpy(&(float){0}"),
        "{target}: {generated}"
    );
    assert!(generated.contains("__builtin_memcpy"), "{generated}");
    assert!(
        !generated.contains("? *(unsigned int *)__builtin_memcpy"),
        "{target}: {generated}"
    );
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn unaligned_word_transfers_keep_float_member_bits_pe32() {
    check("i686-w64-windows-gnu", true, true);
}

#[test]
fn unaligned_word_transfers_keep_float_member_bits_elf64() {
    check("x86_64-unknown-linux-gnu", false, true);
}

#[test]
fn word_stores_from_float_only_members_keep_bits_pe32() {
    check("i686-w64-windows-gnu", true, false);
}
#[test]
fn word_stores_from_float_only_members_keep_bits_elf64() {
    check("x86_64-unknown-linux-gnu", false, false);
}
