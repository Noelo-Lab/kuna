//! Cold-entry evidence must come from an instruction, not an embedded branch encoding.

#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;
use common::process;
use std::path::PathBuf;
use std::process::Command;

struct Fixture {
    dir: PathBuf,
    binary: PathBuf,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn fixture(source: &str, frame: bool) -> Option<Fixture> {
    let Some(cc) = ["gcc", "clang"]
        .into_iter()
        .find(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
    else {
        eprintln!("skipping ELF assembly fixture: no GCC or Clang");
        return None;
    };
    let dir = common::scratch_file("coldentry-source", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let binary = dir.join("fixture");
    let assembly = dir.join("fixture.s");
    let (begin, end) = if frame {
        (".cfi_startproc", ".cfi_endproc")
    } else {
        ("", "")
    };
    std::fs::write(
        &assembly,
        format!(
            ".text\n.globl victim\n.type victim,@function\nvictim:\n.cfi_startproc\n\
         test %edi,%edi\nje .Lsecond\nret\n.Lsecond:\nmov $7,%eax\nret\n\
         .cfi_endproc\n.size victim,.-victim\n.org 0x100\n\
         .globl _start\n.type _start,@function\n_start:\n{begin}\n{source}\n\
         {end}\n.size _start,.-_start\n.section .note.GNU-stack,\"\",@progbits\n"
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
    Some(Fixture { dir, binary })
}

fn entries(fixture: &Fixture, enabled: bool) -> Vec<u64> {
    let (stdout, stderr, rc) = common::run_kuna(&[
        "functions",
        fixture.binary.to_str().unwrap(),
        "--json",
        "--option",
        "coldentry",
        if enabled { "on" } else { "off" },
    ]);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let entries: Vec<_> = parsed["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|function| function["address"].as_u64().unwrap())
        .collect();
    assert!(
        entries.contains(&0x401000) && entries.contains(&0x401100),
        "{entries:x?}"
    );
    entries
}

#[test]
fn branch_bytes_inside_an_immediate_do_not_create_a_cold_entry() {
    for source in [
        ".byte 0x48,0xb8,0xe9\n.long .Lsecond - (. + 4)\n.byte 0,0,0\nret",
        ".byte 0x48,0xb8,0x0f,0x84\n.long .Lsecond - (. + 4)\n.byte 0,0\nret",
    ] {
        for frame in [true, false] {
            let Some(fixture) = fixture(source, frame) else {
                return;
            };
            assert!(!entries(&fixture, false).contains(&0x401005));
            assert!(
                !entries(&fixture, true).contains(&0x401005),
                "an embedded branch encoding is not an incoming edge: {source}, frame={frame}"
            );
        }
    }
}

#[test]
fn real_rel32_branches_still_discover_cold_entries_with_or_without_source_fdes() {
    for encoding in ["0xe9", "0x0f,0x84", "0x2e,0xe9", "0x2e,0x0f,0x84", "0x0f,0x01,0xe8,0xe9"] {
        for frame in [true, false] {
            let source = format!("test %edi,%edi\n.byte {encoding}\n.long .Lsecond - (. + 4)\nret");
            let Some(fixture) = fixture(&source, frame) else {
                return;
            };
            assert!(!entries(&fixture, false).contains(&0x401005));
            assert!(
                entries(&fixture, true).contains(&0x401005),
                "genuine incoming edge lost: {encoding}, frame={frame}"
            );
        }
    }
}
