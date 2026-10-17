//! AIF must retain LEAs that clear the upper half of an x86-64 register.

#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;
use common::process;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Fixture {
    dir: PathBuf,
    binary: PathBuf,
    native: PathBuf,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn fixture(cc: &str, body: &str) -> Fixture {
    let dir = common::scratch_file("aif-lea-padding", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let assembly = dir.join("fixture.s");
    let binary = dir.join("fixture");
    let native = dir.join("native.s");
    let mut text = String::from(".text\n");
    for i in 0..20 {
        text.push_str(&format!(
            ".globl train{i}\n.type train{i},@function\ntrain{i}:\n{body}\n.size train{i},.-train{i}\n"
        ));
    }
    text.push_str(&format!(
        ".org 0x200\n{body}\n.org 0x300\n.globl _start\n.type _start,@function\n_start:\nret\n\
         .section .note.GNU-stack,\"\",@progbits\n"
    ));
    std::fs::write(&assembly, text).unwrap();
    std::fs::write(
        &native,
        format!(
            ".text\n.globl native\n.type native,@function\nnative:\n{body}\n\
             .section .note.GNU-stack,\"\",@progbits\n"
        ),
    )
    .unwrap();
    process::required_output(
        Command::new(cc)
            .args([
                "-nostdlib",
                "-no-pie",
                "-Wl,--build-id=none",
                "-Wl,-Ttext=0x401000",
            ])
            .arg(&assembly)
            .arg("-o")
            .arg(&binary),
    );
    Fixture {
        dir,
        binary,
        native,
    }
}

fn inventory(binary: &Path) -> Vec<u64> {
    let (stdout, stderr, rc) = common::run_kuna(&[
        "functions",
        binary.to_str().unwrap(),
        "--mode",
        "aggressive",
        "--json",
    ]);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    result["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["address"].as_u64().unwrap())
        .collect()
}

#[test]
fn a_zero_extending_lea_stays_at_the_discovered_entry_and_round_trips() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    let Some(&builder) = compilers.first() else {
        eprintln!("skipping native LEA round trip: no GCC or Clang");
        return;
    };
    for (body, arguments) in [
        ("lea (%esi),%esi\nmov %rsi,%rax\nret", "0, value"),
        ("lea (%r8d),%r8d\nmov %r8,%rax\nret", "0, 0, 0, 0, value"),
    ] {
        let fixture = fixture(builder, body);
        let entries = inventory(&fixture.binary);
        assert!(entries.contains(&0x401200), "LEA entry lost: {entries:x?}");
        assert!(!entries.iter().any(|&at| (0x401201..0x401210).contains(&at)));
        let (code, stderr, rc) = common::run_kuna(&[
            "decompile-all",
            fixture.binary.to_str().unwrap(),
            "--mode",
            "aggressive",
            "--functions",
            "sub_401200",
        ]);
        assert_eq!(rc, 0, "{stderr}\n{code}");
        let parameters = vec!["unsigned long"; arguments.split(',').count()].join(",");
        let driver = fixture.dir.join("roundtrip.c");
        std::fs::write(
            &driver,
            format!(
                "{code}\nextern unsigned long native({parameters});\n\
                 int main(void) {{\n\
                 unsigned long values[] = {{0,1,0xffffffffUL,0x100000000UL,\
                 0x1122334455667788UL,0xffffffffffffffffUL}};\n\
                 for (unsigned i=0;i<sizeof(values)/sizeof(values[0]);++i) {{\n\
                 unsigned long value=values[i];\n\
                 unsigned long expected=native({arguments});\n\
                 if (expected != (unsigned int)value || sub_401200({arguments}) != expected) return 1;\n\
                 }}\nreturn 0;\n}}\n"
            ),
        )
        .unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let executable = fixture.dir.join(format!("roundtrip-{cc}-{level}"));
                process::required_output(
                    Command::new(cc)
                        .args([level, "-no-pie"])
                        .arg(&driver)
                        .arg(&fixture.native)
                        .arg("-o")
                        .arg(&executable),
                );
                process::required_output(&mut Command::new(executable));
            }
        }
    }
}

#[test]
fn full_width_self_lea_padding_still_moves_the_entry_to_the_body() {
    let Some(cc) = ["gcc", "clang"]
        .into_iter()
        .find(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
    else {
        eprintln!("skipping LEA padding fixture: no GCC or Clang");
        return;
    };
    let fixture = fixture(cc, "lea (%rsi),%rsi\nmov %rsi,%rax\nret");
    let entries = inventory(&fixture.binary);
    assert!(
        !entries.contains(&0x401200),
        "self-LEA is padding: {entries:x?}"
    );
    assert!(
        entries.contains(&0x401203),
        "body after padding lost: {entries:x?}"
    );
}
