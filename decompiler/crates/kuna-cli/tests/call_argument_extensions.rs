//! Complete emitted functions preserve integer argument bits and full-width stores.
use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn recovered_integer_call_arguments_match_complete_callee_contracts() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() { return; }
    let have_go = process::optional_output(Command::new("go").arg("version")).is_some();
    let kuna = std::env::var_os("KUNA_TEST_BIN")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let declarations = [
        ("pass_u8", "uint8", "uint8 value"), ("pass_u16", "uint8", "uint8 value"),
        ("pass_u32", "uint8", "uint8 value"), ("same_width", "uint8", "uint8 value"),
        ("pass_signed", "uint8", "int4 value"), ("keep_signed", "uint8", "int4 value"),
        ("signed_u8", "int8", "uint8 value"), ("signed_u16", "int8", "uint8 value"),
        ("signed_u32", "int8", "uint8 value"),
        ("store_u8", "uint8", "uint8 *destination,uint8 value"),
        ("store_u16", "uint8", "uint8 *destination,uint8 value"),
        ("store_u32", "uint8", "uint8 *destination,uint8 value"),
        ("store_signed", "uint8", "uint8 *destination,int4 value"),
        ("pass_signed8", "uint8", "int1 value"), ("pass_signed16", "uint8", "int2 value"),
        ("store_signed8", "uint8", "uint8 *destination,int1 value"),
        ("store_signed16", "uint8", "uint8 *destination,int2 value"),
    ];
    let names = ["consume", "consume_signed", "consume_store"].into_iter()
        .chain(declarations.iter().map(|(name, _, _)| *name)).collect::<Vec<_>>().join(",");
    let bodies = |language: &str| {
        let mut command = Command::new(&kuna);
        command.args(["decompile-all", &common::fixture("call_argument_extensions_x86_64"),
            "--functions", &names, "--option", "protoorder", "types", "--language", language, "--json"]);
        for (name, result, args) in declarations {
            command.args(["--assert", &format!("prototype {name} {result} {name}({args})")]);
        }
        let output = process::required_output(&mut command);
        let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        for assertion in document["assertions"].as_array().unwrap() {
            assert_eq!(assertion["status"], "applied", "{assertion}");
        }
        let functions = document["functions"].as_array().unwrap();
        assert_eq!(functions.len(), 20);
        functions.iter().map(|function| {
            assert!(function["error"].is_null(), "{function}");
            function["code"].as_str().unwrap()
        }).collect::<Vec<_>>().join("\n")
    };
    let c = bodies("c");
    let rust = bodies("rust");
    let go = have_go.then(|| bodies("go"));
    let patterns: [u64; 16] = [0,0x100,0x7f00,0x8000,0xff00,0x10000,0x7fff00,0x800000,
        0xffff00,0x7fffff00,0x80000000,0xffffff00,0x100000000,0x1ffffff00,
        0x8000000000000000,0xffffffffffffff00];
    let high = patterns.iter().map(|v| format!("0x{v:x}")).collect::<Vec<_>>().join(",");
    let mut expected = String::new();
    for high in patterns { for byte in 0..256 {
        let value = high | byte;
        let low = value & 0xffffffff;
        let signed = if low & 0x80000000 == 0 { low } else { low | 0xffffffff00000000 };
        let row = [value & 255, value & 65535, low, low, signed, value,
            (value & 255) >> 1, (value & 65535) >> 1, low >> 1,
            value & 255, value & 65535, value & 255, value & 65535, low, low, value & 255, value & 65535];
        expected.push_str(&row.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
        expected.push('\n');
    } }
    let c_source = common::scratch_file("call_argument_extensions", "c");
    let rust_source = common::scratch_file("call_argument_extensions", "rs");
    let go_source = common::scratch_file("call_argument_extensions", "go");
    std::fs::write(&c_source, format!(r#"
#include <stdint.h>
#include <stdio.h>
#include <assert.h>
{c}
int main(void) {{
  const uint64_t high[] = {{{high}}};
  for (unsigned p=0;p<16;++p) for (unsigned byte=0;byte<256;++byte) {{
    uint64_t value=high[p]|byte;
    uint64_t row[17]={{pass_u8(value),pass_u16(value),pass_u32(value),
      pass_signed((int32_t)value),keep_signed((int32_t)value),same_width(value),
      signed_u8(value),signed_u16(value),signed_u32(value),
      pass_signed8((int8_t)value),pass_signed16((int16_t)value),0,0,0,0,0,0}};
    uint64_t cells[3]={{0x123456789abcdef0,0,0xfedcba9876543210}};
    for (unsigned k=0;k<6;++k) {{
      cells[1]=0xa5a5a5a5a5a5a5a5;
      switch(k) {{
        case 0: row[11+k]=store_u8(&cells[1],value); break;
        case 1: row[11+k]=store_u16(&cells[1],value); break;
        case 2: row[11+k]=store_u32(&cells[1],value); break;
        case 3: row[11+k]=store_signed(&cells[1],(int32_t)value); break;
        case 4: row[11+k]=store_signed8(&cells[1],(int8_t)value); break;
        default: row[11+k]=store_signed16(&cells[1],(int16_t)value); break;
      }}
      assert(cells[1]==row[11+k]);
      assert(cells[0]==0x123456789abcdef0 && cells[2]==0xfedcba9876543210);
    }}
    for (unsigned k=0;k<17;++k) printf("%s%lu",k?" ":"",(unsigned long)row[k]);
    putchar('\n');
  }}
}}
"#)).unwrap();
    std::fs::write(&rust_source, format!(r#"
{rust}
fn main() {{ unsafe {{
  for high in [{high}] {{ for byte in 0u64..256 {{
    let value:u64=high|byte;
    let mut row=[pass_u8(value),pass_u16(value),pass_u32(value),
      pass_signed(value as i32),keep_signed(value as i32),same_width(value),
      signed_u8(value) as u64,signed_u16(value) as u64,signed_u32(value) as u64,
      pass_signed8(value as i8),pass_signed16(value as i16),0,0,0,0,0,0];
    let mut cells=[0x123456789abcdef0u64,0,0xfedcba9876543210];
    for k in 0..6 {{
      cells[1]=0xa5a5a5a5a5a5a5a5;
      row[11+k]=match k {{
        0=>store_u8(&mut cells[1],value), 1=>store_u16(&mut cells[1],value),
        2=>store_u32(&mut cells[1],value), 3=>store_signed(&mut cells[1],value as i32),
        4=>store_signed8(&mut cells[1],value as i8), _=>store_signed16(&mut cells[1],value as i16)
      }};
      assert_eq!(cells[1],row[11+k]);
      assert_eq!((cells[0],cells[2]),(0x123456789abcdef0,0xfedcba9876543210));
    }}
    println!("{{}}",row.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
  }} }}
}} }}
"#)).unwrap();
    if let Some(go) = go {
        std::fs::write(&go_source, format!(r#"
package main
import "fmt"
{go}
func main() {{
  for _,high:=range []uint64{{{high}}} {{ for byte:=uint64(0);byte<256;byte++ {{
    value:=high|byte
    row:=[]uint64{{pass_u8(value),pass_u16(value),pass_u32(value),
      pass_signed(int32(value)),keep_signed(int32(value)),same_width(value),
      uint64(signed_u8(value)),uint64(signed_u16(value)),uint64(signed_u32(value)),
      pass_signed8(int8(value)),pass_signed16(int16(value)),0,0,0,0,0,0}}
    cells:=[3]uint64{{0x123456789abcdef0,0,0xfedcba9876543210}}
    for k:=0;k<6;k++ {{
      cells[1]=0xa5a5a5a5a5a5a5a5
      switch k {{
        case 0: row[11+k]=store_u8(&cells[1],value)
        case 1: row[11+k]=store_u16(&cells[1],value)
        case 2: row[11+k]=store_u32(&cells[1],value)
        case 3: row[11+k]=store_signed(&cells[1],int32(value))
        case 4: row[11+k]=store_signed8(&cells[1],int8(value))
        case 5: row[11+k]=store_signed16(&cells[1],int16(value))
      }}
      if cells[1]!=row[11+k] || cells[0]!=0x123456789abcdef0 || cells[2]!=0xfedcba9876543210 {{ panic("store width or canary") }}
    }}
    for k,v:=range row {{ if k!=0 {{ fmt.Print(" ") }}; fmt.Print(v) }}
    fmt.Println()
  }} }}
}}
"#)).unwrap();
    }
    let c_binary = c_source.with_extension("out");
    let rust_binary = rust_source.with_extension("out");
    let go_binary = go_source.with_extension("out");
    for optimization in ["0", "2"] {
        process::required_output(Command::new("cc")
            .args(["-std=c11", "-Wall", "-Wextra", "-Werror", &format!("-O{optimization}")])
            .arg(&c_source).arg("-o").arg(&c_binary));
        assert_eq!(process::required_output(&mut Command::new(&c_binary)).stdout, expected.as_bytes());
        process::required_output(Command::new("rustc")
            .args(["--edition=2021", "--crate-name", "call_argument_extensions",
                "-C", &format!("opt-level={optimization}"), "-C", "overflow-checks=yes"])
            .arg(&rust_source).arg("-o").arg(&rust_binary));
        assert_eq!(process::required_output(&mut Command::new(&rust_binary)).stdout, expected.as_bytes());
        if have_go {
            let mut compile = Command::new("go");
            compile.arg("build");
            if optimization == "0" { compile.arg("-gcflags=all=-N -l"); }
            process::required_output(compile.arg("-o").arg(&go_binary).arg(&go_source));
            assert_eq!(process::required_output(&mut Command::new(&go_binary)).stdout, expected.as_bytes());
        }
    }
    for path in [c_source,rust_source,c_binary,rust_binary] { std::fs::remove_file(path).unwrap(); }
    if have_go { for path in [go_source,go_binary] { std::fs::remove_file(path).unwrap(); } }
}
