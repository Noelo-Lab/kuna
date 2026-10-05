#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

mod common;

use common::process;
use std::process::Command;

const ASSEMBLY: &str = include_str!("fixtures/switch_return_paths.S");

#[test]
fn switch_return_paths_preserve_native_writes() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(
        !compilers.is_empty(),
        "a C compiler is required for the native oracle"
    );
    let variants = [
        ("shared", ASSEMBLY.to_owned(), false),
        (
            "distinct_default",
            ASSEMBLY.replace("ja .Lreturn", "ja .Ldefault").replace(
                ".size toggle_dispatch",
                ".Ldefault:\n    ret\n.size toggle_dispatch",
            ),
            false,
        ),
        (
            "duplicate_return",
            ASSEMBLY.replace("cmpl $3", "cmpl $4").replace(
                "    .long .Llast - 0x400000",
                "    .long .Llast - 0x400000\n    .long .Lreturn - 0x400000",
            ),
            false,
        ),
        (
            "distinct_last",
            ASSEMBLY.replace(".Lreturn:\n", "    ret\n.Lreturn:\n"),
            false,
        ),
        (
            "signed_guard",
            ASSEMBLY
                .replace(
                    "    cmpl $3, %esi",
                    "    testl %esi, %esi\n    js .Lreturn\n    cmpl $3, %esi",
                )
                .replace("ja .Lreturn", "jg .Lreturn"),
            false,
        ),
        (
            "post_switch_write",
            ASSEMBLY
                .replace("    ret\n.Lsecond", "    jmp .Lreturn\n.Lsecond")
                .replace("    ret\n.Llast", "    jmp .Lreturn\n.Llast")
                .replace(".Lreturn:\n", ".Lreturn:\n    addb $1, 3(%rdi)\n"),
            true,
        ),
    ];
    for (name, assembly, post_write) in variants {
        let dir = tempfile::tempdir().unwrap();
        let asm = dir.path().join("input.S");
        let elf = dir.path().join("input.elf");
        std::fs::write(&asm, &assembly).unwrap();
        process::required_output(
            Command::new(compilers[0])
                .args([
                    "-nostdlib",
                    "-no-pie",
                    "-Wl,--build-id=none",
                    "-Wl,-Ttext=0x401000",
                    "-Wl,-e,toggle_dispatch",
                ])
                .arg(&asm)
                .arg("-o")
                .arg(&elf),
        );
        let symbols =
            process::required_output(Command::new("nm").args(["-S", "--defined-only"]).arg(&elf));
        let symbols = String::from_utf8(symbols.stdout).unwrap();
        let size = symbols
            .lines()
            .find_map(|line| {
                let fields: Vec<_> = line.split_whitespace().collect();
                (fields.len() == 4 && fields[3] == "toggle_dispatch")
                    .then(|| u64::from_str_radix(fields[1], 16).unwrap())
            })
            .unwrap();
        let bounds = format!("function 0x401000-{:#x}=toggle_dispatch", 0x401000 + size);
        let prototype = if name == "signed_guard" {
            "prototype toggle_dispatch void toggle_dispatch(struct ToggleFlags *flags, int tag)"
        } else {
            "prototype toggle_dispatch void toggle_dispatch(struct ToggleFlags *flags, unsigned int tag)"
        };
        let specs = common::repo_root().join("specs");
        for region in ["off", "on"] {
            for duplicate in ["off", "on"] {
                let (stdout, stderr, rc) = common::run_kuna(&[
                    "decompile-all", elf.to_str().unwrap(), "--functions", "toggle_dispatch",
                    "--mode", "reliable", "--assert-strict", "--json",
                    "--sleighpath", specs.to_str().unwrap(),
                    "--assert", "typedef struct ToggleFlags { bool first; bool second; bool last; unsigned char post; };",
                    "--assert", &bounds, "--assert", prototype,
                    "--option", "regionstructure", region,
                    "--option", "returndup", duplicate,
                    "--option", "switchreturn", "off",
                ]);
                assert_eq!(rc, 0, "{name}: {stderr}");
                let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
                assert!(
                    doc["assertions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|a| a["status"] != "rejected"),
                    "{stdout}"
                );
                let functions = doc["functions"].as_array().unwrap();
                assert_eq!(functions.len(), 1, "{stdout}");
                assert!(functions[0]["error"].is_null(), "{stdout}");
                let printed = functions[0]["code"].as_str().unwrap();
                let source = dir.path().join("printed.c");
                std::fs::write(
                    &source,
                    format!(
                        r#"
#include <assert.h>
#include <stdbool.h>
#include <string.h>
typedef unsigned int uint4;
typedef int int4;
typedef struct ToggleFlags {{ unsigned char first, second, last, post; }} ToggleFlags;
extern void native_dispatch(ToggleFlags *, unsigned int);
{printed}
int main(void) {{
    const unsigned tags[] = {{0,1,2,3,4,5,0x7fffffff,0x80000000,0xffffffff}};
    for (unsigned mask = 0; mask < 8; ++mask)
        for (unsigned p = 0; p < 2; ++p)
            for (unsigned j = 0; j < sizeof(tags)/sizeof(*tags); ++j) {{
                unsigned tag = tags[j];
                ToggleFlags native = {{mask&1,(mask>>1)&1,(mask>>2)&1,p ? 255 : 0}};
                ToggleFlags emitted = native, expected = native;
                if (tag == 0) expected.first = !expected.first;
                if (tag == 2) expected.second = !expected.second;
                if (tag == 3) expected.last = !expected.last;
                if ({post_write}) ++expected.post;
                native_dispatch(&native, tag);
                toggle_dispatch(&emitted, tag);
                assert(!memcmp(&native, &expected, sizeof expected));
                assert(!memcmp(&emitted, &expected, sizeof expected));
            }}
    return 0;
}}
"#,
                        post_write = u8::from(post_write)
                    ),
                )
                .unwrap();
                let native = dir.path().join("native.S");
                std::fs::write(
                    &native,
                    assembly.replace("toggle_dispatch", "native_dispatch"),
                )
                .unwrap();
                for cc in &compilers {
                    for level in ["-O0", "-O2"] {
                        let exe = dir.path().join("round-trip");
                        process::required_output(
                            Command::new(cc)
                                .args([level, "-no-pie"])
                                .arg(&source)
                                .arg(&native)
                                .arg("-o")
                                .arg(&exe),
                        );
                        let out = Command::new(&exe).output().unwrap();
                        assert!(out.status.success(), "{name}, region={region}, returndup={duplicate}, {cc} {level}: {}\n{printed}\n{stderr}", String::from_utf8_lossy(&out.stderr));
                    }
                }
            }
        }
    }
}
