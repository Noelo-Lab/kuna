//! Asserted Boolean consumers preserve the C backend's byte conversion semantics.
use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn boolean_conversions_preserve_narrowing_and_single_call_evaluation() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let kuna = std::env::var_os("KUNA_TEST_BIN")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let declarations = [
        "prototype as_unsigned8 bool as_unsigned8(uint1 value)",
        "prototype as_signed8 bool as_signed8(int1 value)",
        "prototype masked8 bool masked8(int1 value)",
        "prototype call_unsigned bool call_unsigned(uint8 value)",
        "prototype counted_unsigned uint8 counted_unsigned(uint8 value)",
        "prototype call_signed bool call_signed(int8 value)",
        "prototype counted_signed int8 counted_signed(int8 value)",
    ];
    let names = "as_unsigned8,as_signed8,masked8,call_unsigned,call_signed";
    let bodies = |language: &str| {
        let mut command = Command::new(&kuna);
        command.args([
            "decompile-all", &common::fixture("rust_integer_conversions_x86_64"),
            "--functions", names, "--language", language, "--json",
        ]);
        for declaration in declarations {
            command.args(["--assert", declaration]);
        }
        let output = process::required_output(&mut command);
        let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        for assertion in document["assertions"].as_array().unwrap() {
            assert_eq!(assertion["status"], "applied", "{assertion}");
        }
        let functions = document["functions"].as_array().unwrap();
        assert_eq!(functions.len(), 5);
        functions.iter().map(|function| {
            assert!(function["error"].is_null(), "{function}");
            function["code"].as_str().unwrap()
        }).collect::<Vec<_>>().join("\n")
    };
    let c = bodies("c");
    let rust = bodies("rust");
    assert!(!rust.contains("as bool"), "{rust}");
    assert_eq!(rust.matches("counted_unsigned(value)").count(), 1);
    assert_eq!(rust.matches("counted_signed(value)").count(), 1);
    let c_source = common::scratch_file("rust_boolean_reference", "c");
    let rust_source = common::scratch_file("rust_boolean_emitted", "rs");
    std::fs::write(&c_source, format!(r#"
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>
static unsigned calls;
unsigned long counted_unsigned(unsigned long value) {{ ++calls; return value; }}
long counted_signed(long value) {{ ++calls; return value; }}
{c}
int main(void) {{
  const uint64_t high[] = {{0,0x100,0x8000000000000000ULL,0xffffffffffffff00ULL}};
  for (unsigned p=0;p<4;++p) for (unsigned byte=0;byte<256;++byte) {{
    uint64_t word=high[p]|byte; int64_t signed_word; signed char signed_byte;
    unsigned char low=(unsigned char)byte;
    memcpy(&signed_word,&word,8); memcpy(&signed_byte,&low,1);
    calls=0;
    bool a=as_unsigned8(low),b=as_signed8(signed_byte),c=masked8(signed_byte);
    bool d=call_unsigned(word),e=call_signed(signed_word);
    assert(a==(byte!=0) && b==(byte!=0) && c==((byte&0x55)!=0));
    assert(d==(byte!=0) && e==(byte!=0) && calls==2);
    printf("%u %u %u %u %u %u\n",a,b,c,d,e,calls);
  }}
}}
"#)).unwrap();
    std::fs::write(&rust_source, format!(r#"
use std::sync::atomic::{{AtomicUsize,Ordering}};
static CALLS:AtomicUsize=AtomicUsize::new(0);
unsafe fn counted_unsigned(value:u64)->u64 {{CALLS.fetch_add(1,Ordering::SeqCst);value}}
unsafe fn counted_signed(value:i64)->i64 {{CALLS.fetch_add(1,Ordering::SeqCst);value}}
{rust}
fn main() {{unsafe {{
  for high in [0u64,0x100,0x8000000000000000,0xffffffffffffff00] {{
    for byte in 0u64..256 {{
      let word=high|byte;CALLS.store(0,Ordering::SeqCst);
      let a=as_unsigned8(byte as u8);let b=as_signed8(byte as i8);
      let c=masked8(byte as i8);let d=call_unsigned(word);let e=call_signed(word as i64);
      let calls=CALLS.load(Ordering::SeqCst);
      assert_eq!((a,b,c,d,e,calls),(byte!=0,byte!=0,byte&0x55!=0,byte!=0,byte!=0,2));
      println!("{{}} {{}} {{}} {{}} {{}} {{}}",a as u8,b as u8,c as u8,d as u8,e as u8,calls);
    }}
  }}
}}}}
"#)).unwrap();
    let c_binary = c_source.with_extension("out");
    let rust_binary = rust_source.with_extension("out");
    let mut expected = None;
    for optimization in ["0", "2"] {
        process::required_output(Command::new("cc")
            .args(["-std=c11", "-Wall", "-Wextra", "-Werror", &format!("-O{optimization}")])
            .arg(&c_source).arg("-o").arg(&c_binary));
        let reference = process::required_output(&mut Command::new(&c_binary)).stdout;
        assert_eq!(String::from_utf8_lossy(&reference).lines().count(), 1024);
        if let Some(previous) = &expected { assert_eq!(&reference, previous); }
        process::required_output(Command::new("rustc")
            .args(["--edition=2021", "--crate-name", "rust_boolean_emitted", "-C", &format!("opt-level={optimization}"), "-C", "overflow-checks=yes"])
            .arg(&rust_source).arg("-o").arg(&rust_binary));
        let actual = process::required_output(&mut Command::new(&rust_binary)).stdout;
        assert_eq!(actual, reference);
        expected = Some(reference);
    }
    for path in [c_source,rust_source,c_binary,rust_binary] {
        std::fs::remove_file(path).unwrap();
    }
}
