//! Typed aggregate field stores remain observable by the next call.
use crate::common;
use std::process::Command;

#[test]
fn emitted_partial_global_record_preserves_every_field() {
    check_record("float a; float b; float c; float d;", &["a", "b", "c", "d"]);
}

#[test]
fn emitted_global_array_members_preserve_every_field() {
    check_record(
        "float first[2]; float second[2];",
        &["first[0]", "first[1]", "second[0]", "second[1]"],
    );
}

fn check_record(members: &str, fields: &[&str]) {
    let declaration = format!("typedef struct Record {{ {members} }} Record;");
    let bytes = "9de3bf9021000004a014210080a6200012800006010000004000007e901000101080000c01000000400000769007bff080a220000280000701000000d41fbff0d43c0000d81fbff810bffff4d83c200881c7e00881e80000";
    let mut image = vec![0; 0x220];
    for i in (0..bytes.len()).step_by(2) {
        image[i / 2] = u8::from_str_radix(&bytes[i..i + 2], 16).unwrap();
    }
    let path = common::scratch_file("global-aggregate-store", "bin");
    std::fs::write(&path, image).unwrap();
    let run = |option: Option<&str>| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.arg("decompile").arg(&path).args([
            "0x1000",
            "--raw-image",
            "--base",
            "0x1000",
            "--target",
            "sparc:BE:32:default",
            "--define-function",
            "0x1000-0x1058=update",
            "--define-function",
            "0x1200-0x1208=fill",
            "--define-function",
            "0x1210-0x1218=publish",
            "--assert",
            &declaration,
            "--assert",
            "prototype update void update(int enabled)",
            "--assert",
            "prototype fill int fill(Record *value)",
            "--assert",
            "prototype publish void publish(Record *value)",
            "--assert",
            "data 0x1100 Record cache",
            "--assert-strict",
            "--json",
        ]);
        if let Some(option) = option {
            command.args(["--option", "globalfieldstore", option]);
            if option == "off" {
                command.args(["--option", "pairrefine", "off"]);
            }
        }
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(document["functions"][0]["error"].is_null(), "{document}");
        document
    };
    let document = run(None);
    let enabled = run(Some("on"));
    let disabled = run(Some("off"));
    assert_eq!(
        document["functions"][0]["code"],
        enabled["functions"][0]["code"]
    );
    let legacy = disabled["functions"][0]["code"].as_str().unwrap();
    assert!(
        legacy.contains("cache_") || legacy.contains("cache = v"),
        "{legacy}"
    );
    let body = document["functions"][0]["code"].as_str().unwrap();
    for member in fields {
        assert!(body.contains(&format!("cache.{member} =")), "{body}");
    }
    let source = common::scratch_file("global-aggregate-store", "c");
    std::fs::write(&source, format!(r#"
#include <stdint.h>
#include <string.h>
{declaration}
Record cache;
static uint32_t wanted[4];
static int calls, fills, succeeds;
static uint32_t expected[4];
int fill(Record *value) {{ ++fills; memcpy(value,wanted,sizeof wanted); return succeeds; }}
void publish(Record *value) {{ if(value!=&cache || memcmp(value,expected,sizeof expected)) calls=-100; else ++calls; }}
{body}
int main(void) {{
  static const uint32_t bits[]={{0,0x80000000,1,0x007fffff,0x00800000,0x3f800000,0x7f800000,0xff800000,0x7fc12345,0xffffffff}};
  for(unsigned i=0;i<1000;++i) {{
    for(unsigned j=0;j<4;++j) {{
      uint32_t word=bits[(i+j)%10] ^ (i<10 ? 0 : i);
      if((word&0x7f800000)==0x7f800000 && (word&0x007fffff)) word|=0x00400000;
      wanted[j]=word;
    }}
    for(int enabled=0; enabled<2; ++enabled) for(succeeds=0; succeeds<2; ++succeeds) {{
      memset(&cache,0xa5,sizeof cache); memcpy(expected,&cache,sizeof expected);
      if(enabled && succeeds) memcpy(expected,wanted,sizeof expected);
      calls=0; fills=0; update(enabled);
      if(calls!=(enabled && !succeeds ? 0 : 1) || fills!=enabled || memcmp(&cache,expected,sizeof expected)) return 1;
    }}
  }}
  return 0;
}}
"#)).unwrap();
    for compiler in ["gcc", "clang"] {
        if Command::new(compiler).arg("--version").output().is_err() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let exe = common::scratch_file("global-aggregate-store", "exe");
            let built = Command::new(compiler)
                .args(["-std=c11", level])
                .arg(&source)
                .arg("-o")
                .arg(&exe)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{}",
                String::from_utf8_lossy(&built.stderr)
            );
            assert!(
                Command::new(&exe).status().unwrap().success(),
                "{compiler} {level}: {body}"
            );
            let _ = std::fs::remove_file(exe);
        }
    }
    let _ = std::fs::remove_file(source);
    let _ = std::fs::remove_file(path);
}
