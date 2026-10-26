//! Recompile stores whose pointers resolve after their memory SSA guards exist.
use crate::common;
use common::process;
use std::collections::BTreeSet;
use std::process::Command;

const FUNCTIONS: &str = "s7,s_repeat,s_alias,s_read,s_branch,s_dead,s_two";
const WANT: &str = "s7 1 92 4 9\ns_repeat 94 94 4 3 7 11\ns_alias 92 92 4 13 9\ns_other 13 92 4 81 9\ns_read 81 92 4 9\ns_branch 1 92 4 9\ns_else 1 68 0 17\ns_dead 92 92 9\ns_two 64 64 4 21\ns_distinct 92 92 4 9\n";

#[test]
fn late_resolved_stores_remain_visible_to_calls_and_aliasing_loads() {
    let source = common::fixture("storecopyeffects.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in [
        "storecopyeffects_clang_O0_aarch64.o",
        "storecopyeffects_clang_O2_aarch64.o",
        "storecopyeffects_gcc_O0_x86_64.o",
        "storecopyeffects_gcc_O2_x86_64.o",
    ] {
        let (printed, stderr, code) = common::run_kuna(&[
            "decompile-all",
            &common::fixture(fixture),
            "--functions",
            FUNCTIONS,
        ]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        let globals: BTreeSet<&str> = printed
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .filter(|s| s.starts_with("dat_"))
            .collect();
        assert_eq!(
            globals.len(),
            1,
            "{fixture}: expected one global:\n{printed}"
        );
        let preamble = format!(
            "extern int gi;\nvoid touch(void);\n#define {} gi\n",
            globals.first().unwrap()
        );
        let src = common::scratch_file("storecopyeffects", "c");
        let exe = common::scratch_file("storecopyeffects", "exe");
        std::fs::write(&src, format!("{preamble}{printed}")).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let compile = Command::new(cc)
                    .args([
                        "-std=gnu11",
                        "-w",
                        level,
                        "-fno-strict-aliasing",
                        "-DSTORECOPY_HARNESS",
                        "-DSTORECOPY_DRIVER",
                    ])
                    .args(common::CC_GCC15_DEMOTE)
                    .arg(&src)
                    .arg(&source)
                    .arg("-o")
                    .arg(&exe)
                    .output()
                    .expect("compile the printed C");
                assert!(
                    compile.status.success(),
                    "{fixture} {cc} {level}:\n{}\n{printed}",
                    String::from_utf8_lossy(&compile.stderr)
                );
                let run = process::required_output(&mut Command::new(&exe));
                assert_eq!(
                    String::from_utf8_lossy(&run.stdout),
                    WANT,
                    "{fixture} {cc} {level}: memory writes differ from the source:\n{printed}"
                );
            }
        }
        let _ = std::fs::remove_file(src);
        let _ = std::fs::remove_file(exe);
    }
}

/// Authored SPARC: read the target, sum independent globals while publishing
/// each input through a pointer, then store the sum through a frame-spilled
/// target pointer in a conditional branch's delay slot.
fn global_store_image(count: u32) -> Vec<u8> {
    use object::{write, Architecture, BinaryFormat, Endianness, RelocationFlags, SymbolFlags, SymbolKind, SymbolScope};
    let mut words: Vec<u32> = vec![0x9de3bf90, 0x270c0020, 0xe804e004];
    for i in 0..count {
        words.extend([0x21000400 + i, 0xe2040000, 0xa8050011, 0xe226c000]);
    }
    let call_offset = (words.len() as u64 + 6) * 4;
    words.extend([0x270c0020, 0xe627bfec, 0xee07bfec, 0x80a66000, 0x02800004,
        0xe825e004, 0x40000000, 0x90100018, 0x81c7e008, 0x81e80000]);
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_be_bytes).collect();
    let mut image = write::Object::new(BinaryFormat::Elf, Architecture::Sparc, Endianness::Big);
    let text = image.section_id(write::StandardSection::Text);
    image.append_section_data(text, &bytes, 4);
    image.add_symbol(write::Symbol {
        name: b"sum_publish".to_vec(), value: 0, size: bytes.len() as u64,
        kind: SymbolKind::Text, scope: SymbolScope::Dynamic, weak: false,
        section: write::SymbolSection::Section(text), flags: SymbolFlags::None,
    });
    let observer = image.add_symbol(write::Symbol {
        name: b"observe".to_vec(), value: 0, size: 0,
        kind: SymbolKind::Unknown, scope: SymbolScope::Dynamic, weak: false,
        section: write::SymbolSection::Undefined, flags: SymbolFlags::None,
    });
    image.add_relocation(text, write::Relocation {
        offset: call_offset, symbol: observer, addend: 0,
        flags: RelocationFlags::Elf { r_type: object::elf::R_SPARC_WDISP30 },
    }).unwrap();
    image.write().unwrap()
}

#[test]
fn an_uncovered_global_store_is_not_removed_when_its_pointer_resolves() {
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "a C compiler is required");
    let root = tempfile::tempdir().unwrap();
    for count in [2, 23] {
        let image = root.path().join(format!("store-{count}.o"));
        std::fs::write(&image, global_store_image(count)).unwrap();
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.args(["decompile", image.to_str().unwrap(), "sum_publish", "--json",
            "--assert", "prototype sum_publish void sum_publish(int value,int enabled,int unused,unsigned int *shadow)",
            "--assert", "prototype observe void observe(int value)",
            "--assert", "data 0x30008004 unsigned int target"]);
        for i in 0..count {
            command.arg("--assert").arg(format!("data {:#x} unsigned int input{i}", 0x100000 + i * 1024));
        }
        let output = process::required_output(&mut command);
        let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(document["functions"][0]["error"].is_null(), "{document}");
        let body = document["functions"][0]["code"].as_str().unwrap();
        let globals = (0..count).map(|i| format!("unsigned int input{i};")).collect::<Vec<_>>().join("\n");
        let setup = (0..count).map(|i| format!("input{i}=seed=seed*1664525u+1013904223u; expected+=input{i};")).collect::<Vec<_>>().join("\n");
        let last = count - 1;
        let source = root.path().join(format!("store-{count}.c"));
        std::fs::write(&source, format!(r#"
#include <stdint.h>
{globals}
unsigned int target;
static int calls;
void observe(int value) {{ if(value!=17) calls=-100; else ++calls; }}
{body}
int main(void) {{
  unsigned int seed=1;
  for(unsigned n=0;n<2000;++n) for(int enabled=0;enabled<2;++enabled) {{
    target=seed=seed*1664525u+1013904223u;
    unsigned int expected=target;
    {setup}
    unsigned int shadow=0;
    calls=0; sum_publish(17,enabled,0,&shadow);
    if(target!=expected || shadow!=input{last} || calls!=enabled) return 1;
  }}
  return 0;
}}
"#)).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let exe = root.path().join(format!("store-{count}-{cc}-{level}"));
                let compile = Command::new(cc).args(["-std=c11", level])
                    .arg(&source).arg("-o").arg(&exe).output().unwrap();
                assert!(compile.status.success(), "{}\n{body}", String::from_utf8_lossy(&compile.stderr));
                assert!(Command::new(&exe).status().unwrap().success(), "{cc} {level}: {body}");
            }
        }
    }
}
