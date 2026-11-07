//! Complete emitted callees and callers agree when only low return bytes survive.
use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn recovered_scalar_call_results_preserve_low_bytes_and_signedness() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let have_go = process::optional_output(Command::new("go").arg("version")).is_some();
    let kuna = std::env::var_os("KUNA_TEST_BIN")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let declarations = [
        ("narrow_u8", "uint1"), ("narrow_i8", "int1"),
        ("narrow_u16", "uint2"), ("narrow_i16", "int2"),
        ("narrow_u32", "uint4"), ("narrow_i32", "int4"),
        ("narrow_bool", "bool"), ("full_width", "uint8"),
        ("shifted_bool", "bool"),
    ];
    let names = ["wide_value", "wide_shift"].into_iter().chain(declarations.iter().map(|(name, _)| *name))
        .collect::<Vec<_>>().join(",");
    let bodies = |language: &str| {
        let mut command = Command::new(&kuna);
        command.args(["decompile-all", &common::fixture("scalar_call_results_x86_64"),
            "--functions", &names, "--option", "protoorder", "types", "--language", language, "--json"]);
        for (name, ty) in declarations {
            command.args(["--assert", &format!("prototype {name} {ty} {name}(bool skip,uint8 value)")]);
        }
        let output = process::required_output(&mut command);
        let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        for assertion in document["assertions"].as_array().unwrap() {
            assert_eq!(assertion["status"], "applied", "{assertion}");
        }
        let functions = document["functions"].as_array().unwrap();
        assert_eq!(functions.len(), 11);
        let code = functions.iter().map(|function| {
            assert!(function["error"].is_null(), "{function}");
            function["code"].as_str().unwrap()
        }).collect::<Vec<_>>().join("\n");
        assert_eq!(code.matches("wide_value(value)").count(), 8, "{code}");
        assert_eq!(code.matches("wide_shift(value)").count(), 1, "{code}");
        code
    };
    let c = bodies("c");
    let rust = bodies("rust");
    let go = have_go.then(|| bodies("go"));
    let c_source = common::scratch_file("scalar_call_results_reference", "c");
    let rust_source = common::scratch_file("scalar_call_results_emitted", "rs");
    let go_source = common::scratch_file("scalar_call_results_emitted", "go");
    let high = "0,0x100,0x10000,0x100000000,0x8000000000000000,0xffffffffffffff00,0x7f00,0x8000,0xff00,0x7fffff00,0x80000000,0xffffff00";
    std::fs::write(&c_source, format!(r#"
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <assert.h>
{c}
int main(void) {{
  const uint64_t high[] = {{{high}}};
  for (unsigned p=0;p<12;++p) for (unsigned byte=0;byte<256;++byte) for (unsigned skip=0;skip<2;++skip) {{
    uint64_t value=high[p]|byte, expected=skip?0:value;
    unsigned a=narrow_u8(skip,value),b=(uint8_t)narrow_i8(skip,value);
    unsigned c=narrow_u16(skip,value),d=(uint16_t)narrow_i16(skip,value);
    unsigned e=narrow_u32(skip,value),f=(uint32_t)narrow_i32(skip,value);
    unsigned g=narrow_bool(skip,value); unsigned long h=full_width(skip,value);
    unsigned q=shifted_bool(skip,value);
    assert(a==(uint8_t)expected && b==a && c==(uint16_t)expected && d==c);
    assert(e==(uint32_t)expected && f==e && g==((uint8_t)expected!=0) && h==expected);
    assert(q==(!skip && (uint8_t)(value>>1)!=0));
    printf("%u %u %u %u %u %u %u %lu %u\n",a,b,c,d,e,f,g,h,q);
  }}
}}
"#)).unwrap();
    std::fs::write(&rust_source, format!(r#"
{rust}
fn main() {{ unsafe {{
  for high in [{high}] {{ for byte in 0u64..256 {{ for skip in [false,true] {{
    let value:u64=high|byte;
    println!("{{}} {{}} {{}} {{}} {{}} {{}} {{}} {{}} {{}}",
      narrow_u8(skip,value),narrow_i8(skip,value) as u8,
      narrow_u16(skip,value),narrow_i16(skip,value) as u16,
      narrow_u32(skip,value),narrow_i32(skip,value) as u32,
      narrow_bool(skip,value) as u8,full_width(skip,value),shifted_bool(skip,value) as u8);
  }} }} }}
}} }}
"#)).unwrap();
    if let Some(go) = go {
        std::fs::write(&go_source, format!(r#"
package main
import "fmt"
{go}
func main() {{
  for _,high:=range []uint64{{{high}}} {{ for byte:=uint64(0);byte<256;byte++ {{ for _,skip:=range []bool{{false,true}} {{
    value:=high|byte
    boolean:=0; if narrow_bool(skip,value) {{ boolean=1 }}
    shifted:=0; if shifted_bool(skip,value) {{ shifted=1 }}
    fmt.Printf("%d %d %d %d %d %d %d %d %d\n",
      narrow_u8(skip,value),uint8(narrow_i8(skip,value)),
      narrow_u16(skip,value),uint16(narrow_i16(skip,value)),
      narrow_u32(skip,value),uint32(narrow_i32(skip,value)),
      boolean,full_width(skip,value),shifted)
  }} }} }}
}}
"#)).unwrap();
    }
    let c_binary = c_source.with_extension("out");
    let rust_binary = rust_source.with_extension("out");
    let go_binary = go_source.with_extension("out");
    let mut expected = None;
    for optimization in ["0", "2"] {
        process::required_output(Command::new("cc")
            .args(["-std=c11", "-Wall", "-Wextra", "-Werror", &format!("-O{optimization}")])
            .arg(&c_source).arg("-o").arg(&c_binary));
        let reference = process::required_output(&mut Command::new(&c_binary)).stdout;
        assert_eq!(String::from_utf8_lossy(&reference).lines().count(), 6144);
        if let Some(previous) = &expected { assert_eq!(&reference, previous); }
        process::required_output(Command::new("rustc")
            .args(["--edition=2021", "--crate-name", "scalar_call_results_emitted",
                "-C", &format!("opt-level={optimization}"), "-C", "overflow-checks=yes"])
            .arg(&rust_source).arg("-o").arg(&rust_binary));
        assert_eq!(process::required_output(&mut Command::new(&rust_binary)).stdout, reference);
        if have_go {
            let mut compile = Command::new("go");
            compile.arg("build");
            if optimization == "0" { compile.arg("-gcflags=all=-N -l"); }
            process::required_output(compile.arg("-o").arg(&go_binary).arg(&go_source));
            assert_eq!(process::required_output(&mut Command::new(&go_binary)).stdout, reference);
        }
        expected = Some(reference);
    }
    for path in [c_source,rust_source,c_binary,rust_binary] {
        std::fs::remove_file(path).unwrap();
    }
    if have_go { for path in [go_source,go_binary] { std::fs::remove_file(path).unwrap(); } }
}
