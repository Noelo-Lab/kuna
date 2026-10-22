//! Explicit pointer members relocate indexed addresses inside mapped globals.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::{Object, ObjectSymbol};
use std::process::Command;

#[test]
fn pe32_descriptor_pointer() {
    check("i686-w64-windows-gnu", true);
}
#[test]
fn pe64_descriptor_pointer() {
    check("x86_64-w64-windows-gnu", true);
}
#[test]
fn pe32_integer_control() {
    check("i686-w64-windows-gnu", false);
}
#[test]
fn pe64_integer_control() {
    check("x86_64-w64-windows-gnu", false);
}

fn check(target: &str, pointer: bool) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("indexed-descriptor-pointer", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(&source,r#"
#define API __declspec(dllexport) __attribute__((noinline))
typedef struct Character {unsigned length;float *data;} Character;
typedef union Descriptor {Character character;void *scalar;} Descriptor;
unsigned char arena[2048];
API float consume(void *opaque){Descriptor *d=opaque;return *d->character.data;}
API float send_slot(int index){Descriptor d;d.character.length=4;d.character.data=(float*)((__UINTPTR_TYPE__)arena+64+(__UINTPTR_TYPE__)index*4);return consume(&d);}
"#).unwrap();
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .args(["-O0", "-shared", "-nostdlib", "-Wl,--entry,0"])
            .arg(&source)
            .arg("-o")
            .arg(&binary),
    );
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let exports = image.exports().unwrap();
    let addr = |name: &[u8]| exports.iter().find(|e| e.name() == name).unwrap().address();
    let entry = addr(b"send_slot");
    let consumer = addr(b"consume");
    let arena = image
        .symbols()
        .find(|s| matches!(s.name(), Ok("arena" | "_arena")))
        .expect("global arena symbol")
        .address();
    let wide = target.starts_with("x86_64");
    let name = if wide { "send_slot" } else { "_send_slot" };
    let word = if wide {
        "unsigned long long"
    } else {
        "unsigned int"
    };
    let data = if pointer { "float *" } else { word };
    let output = root.join("out");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.arg("decompile-project")
        .arg(&binary)
        .args(["--addr", &format!("0x{entry:x}"), "-o"])
        .arg(&output)
        .arg("--sleighpath")
        .arg(common::repo_root().join("specs"))
        .args([
            "--option",
            "stackviews",
            "on",
            "--option",
            "realtypes",
            "on",
            "--option",
            "ctypes",
            "on",
            "--assert-strict",
        ]);
    for fact in [
        format!("prototype 0x{entry:x} float __cdecl send_slot(int index)"),
        format!("prototype 0x{consumer:x} float __cdecl consume(void *opaque)"),
        "typedef struct Arena {unsigned char prefix[64];float bank[496];} Arena;".into(),
        format!("data 0x{arena:x} Arena arena"),
        format!("typedef struct TypedCharacter {{unsigned length;{data} data;}} TypedCharacter;"),
        "typedef union Descriptor {TypedCharacter character;void *scalar;} Descriptor;".into(),
        format!("type {name}::v1 Descriptor"),
    ] {
        cmd.args(["--assert", &fact]);
    }
    process::required_output(&mut cmd);
    let generated = output.join("sample.dll.c");
    let body = std::fs::read_to_string(&generated).unwrap();
    println!("{target} pointer={pointer}\n{body}");
    if pointer {
        assert!(
            !body.contains(&format!("0x{:x}", arena + 64)),
            "fixed binary address escaped pointer relocation:\n{body}"
        );
        assert!(
            !body.contains("(float *)(float *)"),
            "pointer cast repeated:\n{body}"
        );
    } else {
        assert!(
            body.contains(&format!("0x{:x}", arena + 64)),
            "integer control changed:\n{body}"
        );
    }
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .args(["-std=gnu11", "-ffreestanding", "-Werror", "-c"])
            .arg(&generated)
            .arg("-o")
            .arg(root.join("generated.o")),
    );
    // Execute against the host's matching 64-bit pointer model. PE32 is checked
    // with its Windows target so descriptor widths and padding stay meaningful.
    if pointer && wide {
        let driver = output.join("driver.c");
        std::fs::write(&driver,r#"
#include "sample.dll.h"
Arena arena;
static int current;
float consume(void *opaque){Descriptor *d=opaque;if(d->character.length!=4||d->character.data!=arena.bank+current)return -12345;return *d->character.data;}
int main(void){for(int i=0;i<496;i++){current=i;arena.bank[i]=(float)i+0.25f;if(send_slot(i)!=arena.bank[i])return 1;}return 0;}
"#).unwrap();
        for opt in ["-O0", "-O2"] {
            let exe = root.join(format!("check{opt}"));
            process::required_output(
                Command::new("clang")
                    .args(["-std=gnu11", opt])
                    .arg(&generated)
                    .arg(&driver)
                    .arg("-o")
                    .arg(&exe),
            );
            process::required_output(&mut Command::new(exe));
        }
    }
    let _ = std::fs::remove_dir_all(root);
}
