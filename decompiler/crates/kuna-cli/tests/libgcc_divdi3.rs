//! A named DI-mode helper's low return word remains a computed value.
use crate::common;
use object::{write, Architecture, BinaryFormat, Endianness, RelocationFlags, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

fn caller(low_word: bool) -> Vec<u8> {
    let mut image = write::Object::new(BinaryFormat::Elf, Architecture::Sparc, Endianness::Big);
    let text = image.section_id(write::StandardSection::Text);
    // save; mov 62,o0; mov i0,o3; sra i0,31,o2; call; sethi 0x200000,o1;
    // st o1,[i1]; ret; restore. Independently authored, with the call's delay slot.
    let mut words: [u32; 9] = [0x9de3bfa0, 0x9010203e, 0x96100018, 0x953e201f, 0x40000000,
        0x13200000, 0xd2264000, 0x81c7e008, 0x81e80000];
    if !low_word {
        words[6] = 0xd0264000;
    }
    let bytes: Vec<u8> = words.into_iter().flat_map(u32::to_be_bytes).collect();
    image.append_section_data(text, &bytes, 4);
    image.add_symbol(write::Symbol {
        name: b"divide_low".to_vec(), value: 0, size: bytes.len() as u64,
        kind: SymbolKind::Text, scope: SymbolScope::Linkage, weak: false,
        section: write::SymbolSection::Section(text), flags: SymbolFlags::None,
    });
    let helper = image.add_symbol(write::Symbol {
        name: b"__divdi3".to_vec(), value: 0, size: 0, kind: SymbolKind::Unknown,
        scope: SymbolScope::Dynamic, weak: false,
        section: write::SymbolSection::Undefined, flags: SymbolFlags::None,
    });
    image.add_relocation(text, write::Relocation {
        offset: 16, symbol: helper, addend: 0,
        flags: RelocationFlags::Elf { r_type: object::elf::R_SPARC_WDISP30 },
    }).unwrap();
    image.write().unwrap()
}

#[test]
fn signed_di_quotient_low_word_survives_an_opaque_call() {
    let root = tempfile::tempdir().unwrap();
    let image = root.path().join("divide.o");
    std::fs::write(&image, caller(true)).unwrap();
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| Command::new(cc).arg("--version").output().is_ok()).collect();
    assert!(!compilers.is_empty(), "a C compiler is required");
    for mode in ["off", "on"] {
        let (output, error, status) = common::run_kuna(&[
            "decompile", image.to_str().unwrap(), "divide_low", "--json",
            "--option", "libproto", mode,
            "--assert", "prototype divide_low void divide_low(int divisor,unsigned int *result)",
        ]);
        assert_eq!(status, 0, "{error}");
        let document: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(document["functions"][0]["error"].is_null(), "{output}");
        let body = document["functions"][0]["code"].as_str().unwrap();
        let source = root.path().join(format!("{mode}.c"));
        std::fs::write(&source, format!(r#"
#include <stdint.h>
#include <limits.h>
long long __divdi3(long long a,long long b) {{ return a/b; }}
{body}
int main(void) {{
  const int divisors[] = {{1,-1,2,-2,3,-3,7,-7,999,-999,INT_MAX,INT_MIN}};
  for(unsigned i=0;i<sizeof divisors/sizeof *divisors;++i) {{
    unsigned int value=0;
    divide_low(divisors[i],&value);
    if(value!=(uint32_t)(INT64_C(0x3e80000000)/(int64_t)divisors[i])) return 1;
  }}
  return 0;
}}
"#)).unwrap();
        for cc in &compilers {
            for opt in ["-O0", "-O2"] {
                let executable = root.path().join(format!("{mode}-{cc}-{opt}"));
                let built = Command::new(cc).args(["-std=c11", opt, "-Werror=uninitialized"])
                    .arg(&source).arg("-o").arg(&executable).output().unwrap();
                if mode == "off" {
                    assert!(!built.status.success(), "legacy helper call unexpectedly compiled: {body}");
                } else {
                    assert!(built.status.success(), "{}\n{body}", String::from_utf8_lossy(&built.stderr));
                    assert!(Command::new(&executable).status().unwrap().success(), "{cc} {opt}: {body}");
                }
            }
        }
    }
}

#[test]
fn an_explicit_scalar_signature_overrides_the_runtime_contract() {
    let root = tempfile::tempdir().unwrap();
    let image = root.path().join("scalar.o");
    std::fs::write(&image, caller(false)).unwrap();
    let (output, error, status) = common::run_kuna(&[
        "decompile", image.to_str().unwrap(), "divide_low", "--json",
        "--assert", "prototype divide_low void divide_low(int value,unsigned int *result)",
        "--assert", "prototype __divdi3 unsigned int __divdi3(unsigned int a,unsigned int b,unsigned int c,unsigned int d)",
    ]);
    assert_eq!(status, 0, "{error}");
    let document: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert!(document["functions"][0]["error"].is_null(), "{output}");
    let body = document["functions"][0]["code"].as_str().unwrap();
    let source = root.path().join("scalar.c");
    std::fs::write(&source, format!(r#"
#include <stdint.h>
#include <limits.h>
unsigned int __divdi3(unsigned int a,unsigned int b,unsigned int c,unsigned int d) {{ return a^b^c^d; }}
{body}
int main(void) {{
  const int inputs[] = {{0,1,-1,37,-37,INT_MAX,INT_MIN}};
  for(unsigned i=0;i<sizeof inputs/sizeof *inputs;++i) {{
    unsigned int value=0;
    divide_low(inputs[i],&value);
    uint32_t sign=inputs[i]<0 ? UINT32_MAX : 0;
    if(value!=(UINT32_C(62)^UINT32_C(0x80000000)^sign^(uint32_t)inputs[i])) return 1;
  }}
  return 0;
}}
"#)).unwrap();
    let mut checked = 0;
    for cc in ["gcc", "clang"] {
        if Command::new(cc).arg("--version").output().is_err() {
            continue;
        }
        let executable = root.path().join(cc);
        let built = Command::new(cc).args(["-std=c11", "-O2", "-Werror=uninitialized"])
            .arg(&source).arg("-o").arg(&executable).output().unwrap();
        assert!(built.status.success(), "{}\n{body}", String::from_utf8_lossy(&built.stderr));
        assert!(Command::new(&executable).status().unwrap().success(), "{cc}: {body}");
        checked += 1;
    }
    assert!(checked > 0, "a C compiler is required");
}
