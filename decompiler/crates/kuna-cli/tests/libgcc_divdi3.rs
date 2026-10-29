//! Signed DI-mode call results survive stores and full-width caller returns
//! under `libgccsigs`, which is off by default, and a caller that consumes a
//! declared wide result does not return it.
use crate::common;
use object::{
    write, Architecture, BinaryFormat, Endianness, RelocationFlags, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::process::Command;

fn caller(low_word: bool) -> Vec<u8> {
    let mut image = write::Object::new(BinaryFormat::Elf, Architecture::Sparc, Endianness::Big);
    let text = image.section_id(write::StandardSection::Text);
    // save; mov 62,o0; mov i0,o3; sra i0,31,o2; call; sethi 0x200000,o1;
    // st o1,[i1]; ret; restore. Independently authored, with the call's delay slot.
    let mut words: [u32; 9] = [
        0x9de3bfa0, 0x9010203e, 0x96100018, 0x953e201f, 0x40000000, 0x13200000, 0xd2264000,
        0x81c7e008, 0x81e80000,
    ];
    if !low_word {
        words[6] = 0xd0264000;
    }
    let bytes: Vec<u8> = words.into_iter().flat_map(u32::to_be_bytes).collect();
    image.append_section_data(text, &bytes, 4);
    image.add_symbol(write::Symbol {
        name: b"divide_low".to_vec(),
        value: 0,
        size: bytes.len() as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: write::SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    let helper = image.add_symbol(write::Symbol {
        name: b"__divdi3".to_vec(),
        value: 0,
        size: 0,
        kind: SymbolKind::Unknown,
        scope: SymbolScope::Dynamic,
        weak: false,
        section: write::SymbolSection::Undefined,
        flags: SymbolFlags::None,
    });
    image
        .add_relocation(
            text,
            write::Relocation {
                offset: 16,
                symbol: helper,
                addend: 0,
                flags: RelocationFlags::Elf {
                    r_type: object::elf::R_SPARC_WDISP30,
                },
            },
        )
        .unwrap();
    image.write().unwrap()
}

#[test]
fn signed_di_quotient_low_word_survives_an_opaque_call() {
    let root = tempfile::tempdir().unwrap();
    let image = root.path().join("divide.o");
    std::fs::write(&image, caller(true)).unwrap();
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| Command::new(cc).arg("--version").output().is_ok())
        .collect();
    assert!(!compilers.is_empty(), "a C compiler is required");
    let decompile = |option: Option<&str>| {
        let mut args = vec![
            "decompile",
            image.to_str().unwrap(),
            "divide_low",
            "--json",
            "--assert",
            "prototype divide_low void divide_low(int divisor,unsigned int *result)",
        ];
        if let Some(value) = option {
            args.extend(["--option", "libgccsigs", value]);
        }
        let (output, error, status) = common::run_kuna(&args);
        assert_eq!(status, 0, "{error}");
        let document: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(document["functions"][0]["error"].is_null(), "{output}");
        document["functions"][0]["code"].as_str().unwrap().to_string()
    };
    let off = decompile(Some("off"));
    assert_eq!(decompile(None), off, "libgccsigs is off by default");
    assert!(
        !off.contains("CONCAT44") && off.contains("__divdi3(0x3e,0x80000000,"),
        "off keeps the inferred four-word call: {off}"
    );
    let body = decompile(Some("on"));
    assert!(body.contains("__divdi3(0x3e80000000,"), "{body}");
    let source = root.path().join("quotient.c");
    std::fs::write(
        &source,
        format!(
            r#"
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
"#
        ),
    )
    .unwrap();
    for cc in &compilers {
        for opt in ["-O0", "-O2"] {
            let executable = root.path().join(format!("{cc}-{opt}"));
            let built = Command::new(cc)
                .args(["-std=c11", opt, "-Werror=uninitialized"])
                .arg(&source)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{}\n{body}",
                String::from_utf8_lossy(&built.stderr)
            );
            assert!(
                Command::new(&executable).status().unwrap().success(),
                "{cc} {opt}: {body}"
            );
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
        "--option", "libgccsigs", "on",
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
        let built = Command::new(cc)
            .args(["-std=c11", "-O2", "-Werror=uninitialized"])
            .arg(&source)
            .arg("-o")
            .arg(&executable)
            .output()
            .unwrap();
        assert!(
            built.status.success(),
            "{}\n{body}",
            String::from_utf8_lossy(&built.stderr)
        );
        assert!(
            Command::new(&executable).status().unwrap().success(),
            "{cc}: {body}"
        );
        checked += 1;
    }
    assert!(checked > 0, "a C compiler is required");
}

#[test]
fn declared_wide_results_survive_caller_return_recovery() {
    if Command::new("clang").arg("--version").output().is_err() {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let fixture = root.path().join("returns.c");
    std::fs::write(
        &fixture,
        r#"
void g(void);
long long q(long long a,long long b){return a/b;}
long long keep(long long a,long long b){long long r=a/b;g();return r;}
long long guard(long long a,long long b){if(b==0)return -1;return a/b;}
long long choose(long long a,long long b,int flag){if(flag)return a/b;return 0x123456789abcdefLL;}
long long mixed(long long a,long long b,int flag){long long r=a/b;if(flag)r=(unsigned int)r;return r;}
int low(long long a,long long b){return (int)(a/b);}
int high(long long a,long long b){return (int)((a/b)>>32);}
"#,
    )
    .unwrap();
    for level in ["-O0", "-O2"] {
        let image = root.path().join("returns.o");
        common::process::required_output(
            Command::new("clang")
                .args(["-target", "sparc-linux-gnu", "-integrated-as", level, "-c"])
                .arg(&fixture)
                .arg("-o")
                .arg(&image),
        );
        for explicit in [false, true] {
            let mut bodies = String::new();
            for function in ["q", "keep", "guard", "low", "high", "choose", "mixed"] {
                let mut args = vec!["decompile", image.to_str().unwrap(), function, "--json"];
                if !explicit {
                    args.extend(["--option", "libgccsigs", "on"]);
                } else {
                    args.extend([
                        "--option",
                        "libproto",
                        "off",
                        "--assert",
                        "prototype __divdi3 long long __divdi3(long long a,long long b)",
                    ]);
                }
                let (output, error, status) = common::run_kuna(&args);
                assert_eq!(status, 0, "{error}");
                let document: serde_json::Value = serde_json::from_str(&output).unwrap();
                assert!(document["functions"][0]["error"].is_null(), "{output}");
                bodies.push_str(document["functions"][0]["code"].as_str().unwrap());
                bodies.push('\n');
            }
            let source = root.path().join("driver.c");
            std::fs::write(&source, format!(r#"
#include <stdint.h>
#include <limits.h>
#include <stdio.h>
#define CONCAT44(hi,lo) (((uint64_t)(uint32_t)(hi)<<32)|(uint32_t)(lo))
static unsigned calls, effects;
long long __divdi3(long long a,long long b) {{ ++calls; return a/b; }}
void g(void) {{ ++effects; }}
{bodies}
#define ARGS(a,b) (uint32_t)((uint64_t)(a)>>32),(uint32_t)(a),(uint32_t)((uint64_t)(b)>>32),(uint32_t)(b)
_Static_assert(sizeof(low(0,0,0,1)) == 4, "the low-word caller returns one word");
_Static_assert(sizeof(high(0,0,0,1)) == 4, "the high-word caller returns one word");
_Static_assert(sizeof(choose(0,0,0,1,0)) == 8, "a nontrivial constant exit stays wide");
_Static_assert(sizeof(mixed(0,0,0,1,0)) == 8, "one changed word does not discard the other");
int main(void) {{
    const int64_t edges[] = {{0,1,-1,2,-3,INT64_MIN,INT64_MAX,INT64_C(0x123456789abcdef),-INT64_C(0x123456789abcdef)}};
    uint64_t random = UINT64_C(0x79b153c5a7d931e5);
    for(unsigned i=0;i<2000;++i) {{
        random ^= random<<13; random ^= random>>7; random ^= random<<17;
        int64_t a = i<81 ? edges[i/9] : (int64_t)random;
        random ^= random<<13; random ^= random>>7; random ^= random<<17;
        int64_t b = i<81 ? edges[i%9] : (int64_t)random;
        if(!b || (a==INT64_MIN && b==-1)) continue;
        uint64_t expected = (uint64_t)(a/b);
        calls = effects = 0;
        if(q(ARGS(a,b)) != expected || calls!=1 || effects) return 1;
        if(keep(ARGS(a,b)) != expected || calls!=2 || effects!=1) return 2;
        if(guard(ARGS(a,b)) != expected || calls!=3 || effects!=1) return 3;
        if(low(ARGS(a,b)) != (uint32_t)expected || calls!=4) return 4;
        if(high(ARGS(a,b)) != (uint32_t)(expected>>32) || calls!=5) return 5;
        if(guard(ARGS(a,0)) != UINT64_MAX || calls!=5 || effects!=1) return 6;
        if(choose(ARGS(a,b),1) != expected || calls!=6 || effects!=1) return 7;
        if(choose(ARGS(a,b),0) != UINT64_C(0x123456789abcdef) || calls!=6) return 8;
        if(mixed(ARGS(a,b),0) != expected || calls!=7) return 9;
        if(mixed(ARGS(a,b),1) != (uint32_t)expected || calls!=8) return 10;
    }}
    return 0;
}}
"#)).unwrap();
            for cc in ["gcc", "clang"] {
                if Command::new(cc).arg("--version").output().is_err() {
                    continue;
                }
                for opt in ["-O0", "-O2"] {
                    let executable = root.path().join(format!("run-{cc}"));
                    let built = Command::new(cc)
                        .args([
                            "-std=c11",
                            opt,
                            "-Werror=uninitialized",
                            "-fsanitize=undefined",
                            "-fno-sanitize-recover=all",
                        ])
                        .arg(&source)
                        .arg("-o")
                        .arg(&executable)
                        .output()
                        .unwrap();
                    assert!(
                        built.status.success(),
                        "{level} explicit={explicit} {cc} {opt}: {}\n{bodies}",
                        String::from_utf8_lossy(&built.stderr)
                    );
                    let ran = Command::new(&executable).output().unwrap();
                    assert!(
                        ran.status.success(),
                        "{level} explicit={explicit} {cc} {opt}: {:?}\n{}\n{bodies}",
                        ran.status.code(),
                        String::from_utf8_lossy(&ran.stderr)
                    );
                }
            }
        }
    }
}

#[test]
fn a_consumed_wide_result_is_not_a_return() {
    if Command::new("clang").arg("--version").output().is_err() {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let fixture = root.path().join("consumed.c");
    std::fs::write(
        &fixture,
        r#"
long long f(long long a);
long long g64;
long long plain(long long a){return f(a);}
void store(long long a,long long *p){*p=f(a);}
void global(long long a){g64=f(a);}
void twice(long long a,long long *p){p[0]=f(a);p[1]=f(a+1);}
"#,
    )
    .unwrap();
    for level in ["-O0", "-O2"] {
        let image = root.path().join("consumed.o");
        common::process::required_output(
            Command::new("clang")
                .args(["-target", "armeb-linux-gnueabi", "-fno-pic", level, "-c"])
                .arg(&fixture)
                .arg("-o")
                .arg(&image),
        );
        for (function, signature) in [
            ("plain", "long long plain("),
            ("store", "void store("),
            ("global", "void global("),
            ("twice", "void twice("),
        ] {
            let (output, error, status) = common::run_kuna(&[
                "decompile", image.to_str().unwrap(), function, "--json",
                "--assert", "prototype f long long f(long long a)",
            ]);
            assert_eq!(status, 0, "{error}");
            let document: serde_json::Value = serde_json::from_str(&output).unwrap();
            let body = document["functions"][0]["code"].as_str().unwrap();
            assert!(body.starts_with(signature), "{level} {function}: {body}");
        }
    }
}
