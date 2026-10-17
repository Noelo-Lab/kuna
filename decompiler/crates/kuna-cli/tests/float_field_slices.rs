use crate::common;
use std::process::Command;

#[test]
fn wide_call_store_preserves_float_field_bits() {
    let mut image = vec![0; 0x201];
    let code = [
        0x48, 0x83, 0xec, 8, 0xe8, 0xf7, 1, 0, 0, 0x66, 0x0f, 0xd6, 5, 0xf3, 0, 0, 0, 0x48, 0x83,
        0xc4, 8, 0xc3,
    ];
    image[..code.len()].copy_from_slice(&code);
    image[0x200] = 0xc3;
    let path = common::scratch_file("float-field-slices", "bin");
    std::fs::write(&path, image).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("decompile")
        .arg(&path)
        .args([
            "0x1000",
            "--raw-image",
            "--base",
            "0x1000",
            "--target",
            "x86:LE:64:default",
            "--define-function",
            "0x1000-0x1016=update",
            "--define-function",
            "0x1200-0x1201=produce",
            "--assert",
            "typedef struct Record { float a; float b; float c; float d; } Record;",
            "--assert",
            "data 0x1100 Record cache",
            "--assert",
            "prototype update void update(void)",
            "--assert-strict",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(document["functions"][0]["error"].is_null(), "{document}");
    let body = document["functions"][0]["code"].as_str().unwrap();
    assert!(!body.contains("SUB84"), "{body}");
    assert!(body.contains("cache.b = ((union"), "{body}");
    assert!(body.contains("cache.c = ((union"), "{body}");
    let source = common::scratch_file("float-field-slices", "c");
    std::fs::write(&source, format!(r#"
#include <stdint.h>
#include <string.h>
typedef uint64_t undefined8;
typedef struct Record {{ float a,b,c,d; }} Record;
static Record cache;
static uint64_t sample;
static unsigned calls;
static undefined8 produce(void) {{ ++calls; return sample; }}
{body}
int main(void) {{
    const uint32_t words[] = {{0,0x80000000,1,0x007fffff,0x00800000,0x3f800000,0xbf800000,0x7f800000,0xff800000,0x7fc12345}};
    for (unsigned i=0;i<10;++i) for (unsigned j=0;j<10;++j) {{
        sample=((uint64_t)words[j]<<32)|words[i];
        memset(&cache,0xa5,sizeof cache);
        calls=0; update();
        uint32_t actual[4]; memcpy(actual,&cache,sizeof actual);
        if(calls!=1 || actual[0]!=0xa5a5a5a5 || actual[1]!=words[i] || actual[2]!=words[j] || actual[3]!=0xa5a5a5a5) return 1;
    }}
    return 0;
}}
"#)).unwrap();
    for compiler in ["gcc", "clang"] {
        if Command::new(compiler).arg("--version").output().is_err() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let executable = common::scratch_file("float-field-slices", "exe");
            let built = Command::new(compiler)
                .args(["-std=c11", "-Wall", "-Wextra", "-Werror", level])
                .arg(&source)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{compiler} {level}: {}\n{body}",
                String::from_utf8_lossy(&built.stderr)
            );
            assert!(
                Command::new(&executable).status().unwrap().success(),
                "{compiler} {level}: {body}"
            );
            std::fs::remove_file(executable).unwrap();
        }
    }
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(path).unwrap();
}
