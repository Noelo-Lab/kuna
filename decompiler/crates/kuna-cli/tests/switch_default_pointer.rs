//! A default target selected before BRANCHIND must remain the switch default.
use crate::common;
use common::process;

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

fn elf(body: &[u8]) -> Vec<u8> {
    let mut image = vec![0; 0x1000];
    image[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
    for (at, value) in [(16, 2u16), (18, 62), (52, 64), (54, 56), (56, 1)] {
        image[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    for (at, value) in [(20, 1u32), (64, 1), (68, 5)] {
        image[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    let size = (0x1000 + body.len()) as u64;
    for (at, value) in [
        (24, 0x401000u64),
        (32, 64),
        (80, 0x400000),
        (88, 0x400000),
        (96, size),
        (104, size),
        (112, 0x1000),
    ] {
        image[at..at + 8].copy_from_slice(&value.to_le_bytes());
    }
    image.extend(body);
    image
}

/// SysV dispatch(unsigned tag, unsigned *counter): commands 0..3 return
/// 10..40. The guard selects a constant default pointer before the same
/// indirect jump. The default optionally shares command 0's body and/or
/// increments *counter before selecting that pointer.
fn fixture(relative: bool, shared: bool, effects: bool, inverted: bool) -> (Vec<u8>, usize) {
    let mut normal = vec![0x48, 0x8d, 0x15, 0, 0, 0, 0];
    if relative {
        normal.extend([0x48, 0x63, 0x04, 0xba, 0x48, 0x01, 0xd0]);
    } else {
        normal.extend([0x48, 0x8b, 0x04, 0xfa]);
    }
    let mut default = Vec::new();
    if effects {
        default.extend([0x83, 0x06, 1]);
    }
    default.extend([0x48, 0x8d, 0x05]);
    let delta = if inverted { normal.len() as i32 + 4 } else { 2 };
    default.extend((delta + if shared { 0 } else { 24 }).to_le_bytes());
    let mut code = vec![0x89, 0xff, 0x83, 0xff, 3];
    let table_lea;
    if inverted {
        code.extend([0x76, (default.len() + 2) as u8]);
        code.extend(&default);
        code.extend([0xeb, normal.len() as u8]);
        table_lea = code.len();
        code.extend(&normal);
    } else {
        code.extend([0x77, (normal.len() + 2) as u8]);
        table_lea = code.len();
        code.extend(&normal);
        code.extend([0xeb, default.len() as u8]);
        code.extend(&default);
    }
    code.extend([0xff, 0xe0]);
    let first_case = code.len();
    for value in [10u32, 20, 30, 40, 0] {
        code.push(0xb8);
        code.extend(value.to_le_bytes());
        code.push(0xc3);
    }
    let end = code.len();
    let table = (end + 7) & !7;
    code.resize(table, 0);
    code[table_lea + 3..table_lea + 7]
        .copy_from_slice(&((table - table_lea - 7) as i32).to_le_bytes());
    for i in 0..4 {
        let target = first_case + 6 * i;
        if relative {
            code.extend(((target as i32) - (table as i32)).to_le_bytes());
        } else {
            code.extend((0x401000u64 + target as u64).to_le_bytes());
        }
    }
    (elf(&code), end)
}

fn decompile(image: &[u8], end: usize) -> String {
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("dispatch.elf");
    std::fs::write(&binary, image).unwrap();
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let mut command = Command::new(
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
    );
    command
        .arg("decompile-all")
        .arg(&binary)
        .args([
            "--addr",
            "0x401000",
            "--json",
            "--mode",
            "reliable",
            "--assert-strict",
            "--sleighpath",
        ])
        .arg(specs)
        .args([
            "--assert",
            &format!("function 0x401000-{:#x}=dispatch", 0x401000 + end),
        ])
        .args([
            "--assert",
            "prototype dispatch int dispatch(unsigned int tag,unsigned int *counter)",
        ]);
    let out = process::output_with_timeout(
        &mut command,
        Duration::from_secs(30),
        Duration::from_millis(20),
    )
    .expect("decompilation timed out");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let doc: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(
        !doc["assertions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|a| a["status"] == "rejected"),
        "{doc}"
    );
    let function = &doc["functions"][0];
    assert!(function["error"].is_null(), "{function}");
    function["code"].as_str().unwrap().to_owned()
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn round_trip(image: &[u8], code: &str, shared: bool, effects: bool) {
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("dispatch.elf");
    std::fs::write(&binary, image).unwrap();
    let source = scratch.path().join("round_trip.c");
    std::fs::write(&source, format!(r#"
#define _GNU_SOURCE
#include <assert.h>
#include <stdbool.h>
#include <fcntl.h>
#include <stdio.h>
#include <sys/mman.h>
#include <unistd.h>
typedef int int4;
typedef unsigned int uint4;
typedef unsigned long uint8;
{code}
int main(int argc,char **argv) {{
    int fd = open(argv[1], O_RDONLY);
    off_t size = lseek(fd, 0, SEEK_END);
    if (mmap((void *)0x400000, size, PROT_READ | PROT_EXEC, MAP_PRIVATE | MAP_FIXED_NOREPLACE, fd, 0) != (void *)0x400000) {{
        perror("mmap"); return 2;
    }}
    int (*native)(unsigned,unsigned *) = (void *)0x401000;
    unsigned extremes[] = {{0x7fffffff,0x80000000,0xbad1abe1,0xfffffffe,0xffffffff}};
    alarm(10);
    for (unsigned i = 0; i < 69; ++i) {{
        unsigned tag = i < 64 ? i : extremes[i-64];
        unsigned a = 41, b = 41;
        int want = tag < 4 ? (tag+1)*10 : {default};
        int original = native(tag, &a);
        assert(original == want);
        assert(a == 41 + ({effects} && tag > 3));
        int emitted = dispatch(tag, &b);
        if (emitted != want || b != a) {{
            printf("tag=%u native=%d emitted=%d counter=%u/%u\n",tag,original,emitted,a,b);
            return 1;
        }}
    }}
    return 0;
}}
"#, default=if shared {10} else {0}, effects=u8::from(effects))).unwrap();
    let exe = scratch.path().join("round_trip");
    let mut found = 0;
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        found += 1;
        for level in ["-O0", "-O2"] {
            process::required_output(
                Command::new(cc)
                    .args(["-fPIE", "-pie", level])
                    .arg(&source)
                    .arg("-o")
                    .arg(&exe),
            );
            process::required_output(Command::new(&exe).arg(&binary));
        }
    }
    assert!(found > 0, "the native oracle requires a C compiler");
}

#[test]
fn selected_default_pointer_has_a_default_case() {
    for relative in [false, true] {
        for shared in [false, true] {
            for effects in [false, true] {
                for inverted in [false, true] {
                    let (image, end) = fixture(relative, shared, effects, inverted);
                    let code = decompile(&image, end);
                    assert!(
                        code.contains("switch(") && code.contains("default:"),
                        "{code}"
                    );
                    for label in 1..4 {
                        assert!(code.contains(&format!("case {label}:")), "{code}");
                    }
                    assert!(!code.to_ascii_lowercase().contains("bad1abe1"), "{code}");
                    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
                    round_trip(&image, &code, shared, effects);
                }
            }
        }
    }
}
